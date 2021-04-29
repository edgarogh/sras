use crate::Event;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fs::read_to_string;
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread::Thread;

pub enum ServerEvent {
    Time(u32),
    Humi(u32),
}

pub struct Server {}

impl Server {
    pub fn new(
        port: u16,
        send: mpsc::Sender<Event>,
        recv: mpsc::Receiver<ServerEvent>,
    ) -> std::io::Result<Self> {
        let listener = match TcpListener::bind(("127.0.0.1", port)) {
            Ok(listener) => listener,
            Err(err) => {
                eprintln!("err={:?}", err);
                return Err(err);
            }
        };

        std::thread::spawn(move || {
            let (mut tcp_rcv, mut tcp_snd) = match listener.accept() {
                Ok((stream, _)) => (stream.try_clone().unwrap(), stream),
                Err(err) => {
                    eprintln!("[server] error: {}", err);
                    return;
                }
            };

            let mut tcp_snd_r = tcp_snd.try_clone().unwrap();

            std::thread::spawn(move || {
                match tcp_rcv.read_u32::<BigEndian>().unwrap() {
                    // Ping
                    0 => tcp_snd_r.write_u32::<BigEndian>(1918).unwrap(),
                    // Dispense
                    1 => {
                        send.send(Event::Dispense(tcp_rcv.read_u32::<BigEndian>().unwrap(), 0))
                            .unwrap();

                        println!("rcv dispense from java")
                    }
                    id => eprintln!("[server] received packet id {}", id),
                }
                send
            });

            loop {
                if let Ok(e) = recv.try_recv() {
                    match e {
                        ServerEvent::Time(time) => {
                            tcp_snd.write_u32::<BigEndian>(1).unwrap();
                            tcp_snd.write_u32::<BigEndian>(time).unwrap();
                        }
                        ServerEvent::Humi(humi) => {
                            tcp_snd.write_u32::<BigEndian>(2).unwrap();
                            tcp_snd.write_u32::<BigEndian>(humi).unwrap();
                        }
                    }
                }
            }
        });

        Ok(Server {})
    }
}
