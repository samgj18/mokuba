use mcore::{
    gen,
    mstd::{codec::Codec, error::DecodeError, executable::Execute, param::PassParams},
    syntax::option::syntax::*,
};
use mmacro::{ConstructorM, DisplayM, VariantM};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, DisplayM, ConstructorM)]
pub struct Save {
    pub user: String,
    pub password: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Eq, DisplayM, VariantM)]
pub enum Command {
    Help,
    Version,
    Password(u32),
    Save(Save),
    Unknown(String, Option<String>),
}

static VERSION: &str = env!("CARGO_PKG_VERSION");
static BIN_NAME: &str = env!("CARGO_PKG_NAME");
static HELP: &str = concat!(
    "Usage: {} [OPTION]...\n",
    env!("CARGO_PKG_NAME"),
    " [command] [options]\n",
);

#[derive(Debug, PartialEq, Eq, DisplayM, ConstructorM)]
struct Args {
    pub first: Option<String>,
    pub second: Option<String>,
    pub third: Option<String>,
    pub fourth: Option<String>,
    pub rest: Vec<String>,
}

fn deconstruct(input: &str) -> Result<Args, DecodeError> {
    Vec::decode(Some(input)).map(|v| {
        let mut iter = v.into_iter();
        Args::new(
            iter.next(),
            iter.next(),
            iter.next(),
            iter.next(),
            iter.collect(),
        )
    })
}

impl Command {
    pub fn parse(input: &str) -> Result<Command, String> {
        deconstruct(input)
            .map_err(|e| e.cause)
            .map(|command| -> Command {
                match command.first.as_deref() {
                    Some("-h") => Command::Help,
                    Some("-v") => Command::Version,
                    Some("-p") => u32::decode(command.second.as_deref())
                        .map_or_else(|e| Command::Unknown(e.cause, None), Command::Password),
                    Some("-s") => {
                        let user = String::decode(command.second.as_deref());
                        let password = String::decode(command.third.as_deref());
                        let url = String::decode(command.fourth.as_deref());
                        match (user, password, url) {
                            (Ok(user), Ok(password), Ok(url)) => {
                                Command::Save(Save::new(user, password, url))
                            }
                            (Err(e1), Err(e2), Err(e3)) => Command::Unknown(
                                format!("{} {} {}", e1.cause, e2.cause, e3.cause),
                                Some(input.to_string()),
                            ),
                            (Err(e), _, _) => Command::Unknown(e.cause, input.to_string().some()),
                            (_, Err(e), _) => Command::Unknown(e.cause, None),
                            (_, _, Err(e)) => Command::Unknown(e.cause, None),
                        }
                    }
                    _ => Command::Unknown(input.to_owned(), command.second),
                }
            })
    }
}

impl Execute<Command, String, Error> for Command {
    fn execute(&self) -> Result<String, Error> {
        match self {
            Command::Unknown(cause, code) => Err(Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unknown command: {} {}",
                    cause,
                    code.as_deref().unwrap_or("")
                ),
            )),
            Command::Help => Ok(HELP.to_owned()),
            Command::Version => Ok(format!("{} version: {}\n", BIN_NAME, VERSION)),
            Command::Password(param) => gen(PassParams::new(param.to_owned()))
                .map(|s| format!("Password generated with length {}: '{}'\n", param, s))
                .map_err(|e| Error::new(std::io::ErrorKind::Other, e.cause)),
            Command::Save(save) => {
                // TODO: Implement - here we should save the password in whatever way we want to
                Ok(format!(
                    "User: {}, password: {}, url: {}\n saved successfully\n",
                    save.user, save.password, save.url
                ))
            }
        }
    }
}
