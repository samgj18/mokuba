use mcore::executable::Execute;
use mcore::gen;
use mcore::model::params::PassParams;
use mmacro::{DisplayM, VariantM};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, DisplayM, VariantM)]
pub enum Command {
    Help,
    Version,
    Password,
}

impl Command {
    pub fn parse(input: &str) -> Result<Command, String> {
        // split the input into words
        let words: Vec<&str> = input.split_whitespace().collect();
        let bin_name = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "".to_string());

        // parse the first word
        match words.first() {
            Some(&"-h") => Ok(Command::Help),
            Some(&"-v") => Ok(Command::Version),
            Some(&"-p") => Ok(Command::Password),
            Some(&"--password") => Ok(Command::Password),
            Some(&"--help") => Ok(Command::Help),
            Some(&"--version") => Ok(Command::Version),
            _ => Err(format!(
                "
            Usage: {} [OPTION]...
            Search for PATTERNS in each FILE.
            Example: {} -p 16
            Unknown command: {}",
                bin_name, bin_name, input
            )),
        }
    }
}

pub enum Executable {
    Help,
    Version,
    Password(PassParams),
}

impl Execute<Executable, String, Error> for Command {
    fn execute(&self) -> Result<String, Error> {
        match self {
            Command::Help => Ok("Help".to_string()),
            Command::Version => Ok("Version".to_string()),
            Command::Password => gen(PassParams::new(16))
                .map(|s| format!("Password generated with length {}: '{}'\n", 16, s))
                .map_err(Error::from),
        }
    }
}
