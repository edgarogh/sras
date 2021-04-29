use super::ClapHelper;
use super::Event;
use crate::plant_config::PlantConfig;
use clap::{Clap, IntoApp};
use rustyline::error::ReadlineError;
use std::fs::read_to_string;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::mpsc;

#[derive(Clap, Debug)]
struct InternalCLI {
    #[clap(subcommand)]
    pub sub: Subcommand,
}

#[derive(Clap, Debug)]
enum Subcommand {
    Logs(Logs),
    Load(Load),
    Dispense(Dispense),
    Calibrate(Calibrate),
    Server(Server),
}

#[derive(Clap, Debug)]
struct Logs;

#[derive(Clap, Debug)]
struct Load {
    file: PathBuf,
}

#[derive(Clap, Debug)]
struct Dispense {
    duration: Option<NonZeroU32>,
    wait: Option<u32>,
}

#[derive(Clap, Debug)]
struct Calibrate {
    #[clap(subcommand)]
    sensor: CalibrateSensor,
}

#[derive(Clap, Debug)]
enum CalibrateSensor {
    HygroSoil(sensor::HygroSoil),
    Thermometer(sensor::Thermometer),
    HygroAmbiant(sensor::HygroAmbiant),
}

#[derive(Clap, Debug)]
enum Bound {
    Low,
    High,
}

mod sensor {
    use clap::Clap;

    #[derive(Clap, Debug)]
    pub struct HygroSoil {
        #[clap(arg_enum)]
        bound: super::Bound,
    }

    #[derive(Clap, Debug)]
    pub struct HygroAmbiant {
        #[clap(arg_enum)]
        bound: super::Bound,
    }

    #[derive(Clap, Debug)]
    pub struct Thermometer {
        #[clap(arg_enum)]
        bound: super::Bound,
    }
}

#[derive(Clap, Debug)]
struct Server {
    port: u16,
}

pub fn shell_loop(events: mpsc::Sender<Event>, logs: mpsc::Receiver<String>) {
    let mut editor = rustyline::Editor::new();
    editor.set_helper(Some(ClapHelper(InternalCLI::into_app())));

    for line in editor.iter(":") {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                if let ReadlineError::Interrupted = err {
                    std::process::exit(0);
                }

                eprintln!("[rl] error reading line: {}", err);
                continue;
            }
        };

        let args = match shell_words::split(&line) {
            Ok(args) => args,
            Err(_) => {
                eprintln!("[rl] unbalanced quotes");
                continue;
            }
        };

        let cmd = match InternalCLI::try_parse_from(std::iter::once(String::from(":")).chain(args))
        {
            Ok(cmd) => cmd,
            Err(err) => {
                eprintln!("[rl] invalid command: {}", err);
                continue;
            }
        };

        match cmd.sub {
            Subcommand::Logs(_) => {
                for messages in logs.try_iter() {
                    println!("[arduino] {}", messages);
                }
            }
            Subcommand::Load(Load { file }) => {
                println!(
                    "[load] loading plant file {}",
                    file.as_os_str().to_string_lossy().as_ref()
                );
                let file = match read_to_string(file) {
                    Ok(file) => file,
                    Err(err) => {
                        eprintln!("[load] couldn't read file: {}", err);
                        continue;
                    }
                };

                let config: PlantConfig = match toml::from_str(&file) {
                    Ok(config) => config,
                    Err(err) => {
                        eprintln!("[load] couldn't parse file: {}", err);
                        continue;
                    }
                };

                println!("[load] uploading \"{}\"", config.name);

                events.send(Event::Configure(Box::new(config))).unwrap();
            }
            Subcommand::Dispense(Dispense { duration, wait }) => events
                .send(Event::Dispense(
                    duration.map(|n| n.get()).unwrap_or_default(),
                    wait.unwrap_or(u32::MAX),
                ))
                .unwrap(),
            Subcommand::Calibrate(_) => todo!(),
            Subcommand::Server(Server { port }) => {
                events.send(Event::StartServer(port)).unwrap();
            }
        }
    }
}
