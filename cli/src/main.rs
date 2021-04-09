mod pcd8544;

use crate::pcd8544::{Glyph, PCD8544};
use clap::Clap;
use serialport::{DataBits, Parity, SerialPortType, StopBits};
use std::convert::TryInto;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc;
use std::time::Duration;

#[derive(Clap)]
struct Args {
    /// Path to a serial port to connect to
    serial_port_path: Option<PathBuf>,
}

#[derive(Debug)]
enum Event {
    NewGlyph(u8, Glyph),
    Write(String),
    SetCursor(usize, usize),
    Clear,
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
        let port = serialport::new(port.to_string_lossy(), 9600)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .timeout(Duration::from_secs(10))
            .open()
            .unwrap();

        let buf_reader = BufReader::new(port);
        let lines = buf_reader.lines();

        let (send, recv) = mpsc::channel();

        std::thread::spawn(move || {
            main_loop(recv);
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
                    println!("[arduino] {:?}", line);
                }
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

fn main_loop(events: mpsc::Receiver<Event>) -> ! {
    let mut window = PCD8544::new(16, 6);

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
                Event::Clear => window.clear(),
            }
        }

        window.update();
    }
}
