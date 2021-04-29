use rustyline::completion::{Candidate, Completer};
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hint, Hinter};
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

pub struct ClapHelper<'help>(pub clap::App<'help>);

/*fn complete(command: &clap::App, args: &[String]) -> Vec<String> {
    let (arg, rest) = match args {
        [] => return Vec::new(),
        [arg, rest @ ..] => (arg, rest),
    };

    let mut suggestions = self
        .0
        .get_subcommands()
        .map(|cmd| cmd.get_name())
        .filter(|c| c.starts_with(start))
        .map(|c| shell_words::quote(c).to_owned())
        .collect::<Vec<String>>();

    match suggestions.as_mut_slice() {
        [single] => single.push(' '),
        _ => (),
    }
}*/

macro_rules! try_default {
    ($expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(_) => return Ok(Default::default()),
        }
    };
}

impl<'help> Completer for ClapHelper<'help> {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let line = &line[..pos];

        if let Some(path) = line.strip_prefix("load ") {
            let read_dir_dir = match path {
                "" | "." => Path::new("."),
                ".." => Path::new(".."),
                "/" => Path::new("/"),
                other => Path::new(other).parent().unwrap(),
            };

            let dir = try_default!(std::fs::read_dir(read_dir_dir));

            return Ok((
                0,
                dir.filter_map(|c| c.ok())
                    .map(|c| c.path().to_string_lossy().to_string())
                    .collect(),
            ));
        }

        /*let args = match shell_words::split(line) {
            Ok(args) => args,
            Err(_) => return Default::default(),
        };

        Ok((0, complete(&self.0, args.as_slice())));*/

        if !line.find(' ').is_some() {
            let mut suggestions = self
                .0
                .get_subcommands()
                .map(|cmd| cmd.get_name())
                .filter(|c| c.starts_with(line))
                .map(|c| shell_words::quote(c).to_string())
                .collect::<Vec<String>>();

            match suggestions.as_mut_slice() {
                [single] => single.push(' '),
                _ => (),
            }

            Ok((0, suggestions))
        } else {
            Ok((0, Vec::with_capacity(0)))
        }
    }
}

impl Hinter for ClapHelper<'_> {
    type Hint = String;
}

impl Highlighter for ClapHelper<'_> {}

impl Validator for ClapHelper<'_> {}

impl Helper for ClapHelper<'_> {}
