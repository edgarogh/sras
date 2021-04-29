mod clap_helper;
mod pcd8544;
mod plant_config;
mod server;
mod shell;

use crate::clap_helper::ClapHelper;
use crate::pcd8544::{Glyph, PCD8544};
use crate::plant_config::PlantConfig;
use crate::server::{Server, ServerEvent};
use crate::shell::shell_loop;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use clap::{Clap, IntoApp};
use rustyline::completion::{Candidate, Completer};
use rustyline::error::ReadlineError;
use rustyline::Helper;
use serialport::{DataBits, Parity, SerialPort, SerialPortType, StopBits};
use std::collections::VecDeque;
use std::convert::TryInto;
use std::fs::read_to_string;
use std::io::{BufRead, BufReader, Write};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

type ArduinoEndianness = byteorder::LittleEndian;

#[derive(Clap)]
struct Args {
    /// Path to a serial port to connect to
    serial_port_path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum Event {
    Configure(Box<PlantConfig>),
    NewGlyph(u8, Glyph),
    Write(String),
    SetCursor(usize, usize),
    Clear,
    Dispense(u32, u32),
    StartServer(u16),
    JTime(u32),
    JHumi(u32),
    Unknown,
}

impl FromStr for Event {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if !s.is_empty() {
            let (command, args) = s.split_at(1);

            match command.chars().next().unwrap() as u8 {
                b'g' => {
                    let args = args
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<_>>();
                    Event::NewGlyph(args[0], Glyph(args[1..].try_into().unwrap()))
                }
                b'p' => Event::Write(args.into()),
                b'm' => {
                    let (x, y) = args.split_once(',').unwrap();
                    Event::SetCursor(x.parse().unwrap(), y.parse().unwrap())
                }
                b'c' => Event::Clear,
                b'1' => Event::JTime(args.parse().unwrap()),
                b'2' => Event::JHumi(args.parse().unwrap()),
                _ => Event::Unknown,
            }
        } else {
            Event::Unknown
        })
    }
}

fn main() {
    let args: Args = Clap::parse();

    if let Some(port) = args.serial_port_path {
        let mut port = serialport::new(port.to_string_lossy(), 9600)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .timeout(Duration::from_secs(10))
            .open()
            .unwrap();

        let mut port_write = port.try_clone().expect("cannot clone port ;(");

        let buf_reader = BufReader::new(&mut port);
        let lines = buf_reader.lines();

        let (send_logs, logs) = mpsc::channel();
        let (send, recv) = mpsc::channel();

        let send2 = send.clone();
        std::thread::spawn(move || {
            main_loop(recv, send2, port_write);
        });

        let send2 = send.clone();
        std::thread::spawn(move || {
            shell_loop(send2, logs);
        });

        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() || line.contains('\0') {
                    continue;
                }

                if let Some(cmd) = line.strip_prefix('+') {
                    match send.send(cmd.parse().unwrap()) {
                        Ok(()) => (),
                        Err(mpsc::SendError(event)) => {
                            eprintln!(
                                "Main loop panicked, couldn't deliver {:?}. Exiting.",
                                &event,
                            );
                            return;
                        }
                    }
                } else {
                    send_logs.send(line).unwrap();
                };
            }
        }
    } else {
        // Serial port list / Autocompletion

        if let Ok(ports) = serialport::available_ports() {
            for port in ports {
                print!("{}", port.port_name);

                if let SerialPortType::UsbPort(usb) = port.port_type {
                    match (usb.manufacturer, usb.product) {
                        (None, None) => (),
                        (Some(m), Some(p)) => print!("\t{} {}", m, p),
                        (Some(m), None) => print!("\t{}", m),
                        (None, Some(p)) => print!("\t{}", p),
                    }
                }

                println!();
            }
        }
    }
}

fn main_loop(
    events: mpsc::Receiver<Event>,
    send: mpsc::Sender<Event>,
    mut port: Box<dyn SerialPort>,
) -> ! {
    let mut window = PCD8544::new(12, 6);

    let (server_snd, server_rcv) = mpsc::channel();
    let mut server_init = Some((send, server_rcv));
    let mut server = None;

    loop {
        if let Ok(event) = events.try_recv() {
            match event {
                Event::Unknown => (),
                Event::NewGlyph(g, glyph) => window.create_char(g, glyph),
                Event::SetCursor(x, y) => window.set_cursor(x, y),
                Event::Write(c) => {
                    for c in c.as_bytes() {
                        window.write(*c)
                    }
                }
                Event::Configure(config) => {
                    port.write(&[b'+', b'c']).unwrap();
                    port.write_u32::<ArduinoEndianness>(0);
                    port.write_u32::<ArduinoEndianness>(1023);
                    port.write_u32::<ArduinoEndianness>(0);
                    port.write_u32::<ArduinoEndianness>(1023);
                    port.write_u32::<ArduinoEndianness>(0);
                    port.write_u32::<ArduinoEndianness>(1023);
                    port.write_u32::<ArduinoEndianness>(config.pump.duration);
                    port.write_u32::<ArduinoEndianness>(config.pump.wait);
                    port.write_u8(b'\n');
                }
                Event::Dispense(duration, wait) => {
                    port.write(&[b'+', b'd']).unwrap();
                    port.write_u32::<ArduinoEndianness>(duration).unwrap();
                    port.write_u32::<ArduinoEndianness>(wait).unwrap();
                    port.write_u8(b'\n').unwrap();
                }
                Event::StartServer(port) => {
                    let (send, server_event_rcv) = server_init.take().unwrap();

                    match Server::new(if port != 0 { port } else { 1918 }, send, server_event_rcv) {
                        Ok(new_server) => {
                            server = Some(new_server);
                        }
                        Err(_) => (),
                    }
                }
                Event::JTime(time) => server_snd
                    .send(ServerEvent::Time(
                        ((1023.0 - (time as f32) / 1023.0) * 12000.0 + 6000.0) as u32,
                    ))
                    .unwrap(),
                Event::JHumi(time) => server_snd.send(ServerEvent::Humi(time)).unwrap(),
                Event::Clear => window.clear(),
            }
        }

        window.update();
    }
}
