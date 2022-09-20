use mcore::executable::Executable;
use mcore::parser::Parser;
use mmacro::{DisplayM, VariantM};
use std::fmt::{Debug, Display, Formatter, Result};
use std::io::Error;
use std::result::Result as StdResult;

#[derive(Debug, PartialEq, Eq, DisplayM, VariantM)]
pub enum Command {
    Help,
    Version,
}

impl Parser<Command> for Command {
    fn parse(input: &str) -> StdResult<Command, String> {
        match input.trim() {
            "help" => Ok(Command::Help),
            "version" => Ok(Command::Version),
            _ => Err(format!("Unknown command: {}", input)),
        }
    }
}

impl Executable<Command, String, Error> for Command {
    fn execute(&self) -> StdResult<String, Error> {
        match self {
            Command::Help => Ok("Help".to_string()),
            Command::Version => Ok("Version".to_string()),
        }
    }
}
