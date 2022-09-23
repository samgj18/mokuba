use mcore::gen;
use mcore::model::params::PassParams;
use mcore::{codec::Codec, executable::Execute};
use mmacro::{ConstructorM, DisplayM, VariantM};
use std::{io::Error, str::FromStr, vec};

#[derive(Debug, PartialEq, Eq, DisplayM, ConstructorM)]
pub struct Save {
    pub user: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, PartialEq, Eq, DisplayM, VariantM)]
pub enum Command {
    Help,
    Version,
    Password(Option<u32>),
    Save(Save),
}

static VERSION: &str = env!("CARGO_PKG_VERSION");
static BIN_NAME: &str = env!("CARGO_PKG_NAME");
static HELP: &str = concat!(
    "Usage: {} [OPTION]...\n",
    env!("CARGO_PKG_NAME"),
    " [command] [options]\n",
);

fn from_vec<T>(input: &[&str]) -> Vec<T>
where
    T: FromStr,
{
    input
        .iter()
        .map(|param| match param.parse::<T>() {
            Ok(param) => param,
            Err(_) => panic!("Error parsing parameter"),
        })
        .collect()
}

impl Command {
    pub fn parse(input: &str) -> Result<Command, String> {
        // TODO: Refactor this function to something more readable and functional
        let args: Vec<&str> = input.split_whitespace().collect();
        let command = args.first();
        let param = args.get(1).copied();

        let options = match args.get(2..) {
            Some(options) => options.to_vec(),
            None => vec![],
        };

        match command {
            Some(&"-h") => Ok(Command::Help),

            Some(&"-v") => Ok(Command::Version),

            Some(&"-p") => Option::decode(param)
                .map(Command::Password)
                .map_err(|e| e.to_string()),

            Some(&"-s") => Ok(Command::Save(Save::new(
                from_vec(&options).get(0).cloned(),
                from_vec(&options).get(1).cloned(),
                from_vec(&options).get(2).cloned(),
            ))),

            Some(&"--password") => Option::decode(param)
                .map(Command::Password)
                .map_err(|e| e.to_string()),

            Some(&"--save") => Ok(Command::Save(Save::new(
                from_vec(&options).get(0).cloned(),
                from_vec(&options).get(1).cloned(),
                from_vec(&options).get(2).cloned(),
            ))),

            Some(&"--help") => Ok(Command::Help),

            Some(&"--version") => Ok(Command::Version),
            _ => Err(format!("{}Unknown command: {}\n", HELP, input)),
        }
    }
}

impl Execute<Command, String, Error> for Command {
    fn execute(&self) -> Result<String, Error> {
        match self {
            Command::Help => Ok(HELP.to_owned()),
            Command::Version => Ok(format!("{} version: {}\n", BIN_NAME, VERSION)),
            Command::Password(param) => gen(PassParams::new(param.unwrap_or(16)))
                .map(|s| {
                    format!(
                        "Password generated with length {}: '{}'\n",
                        param.unwrap_or(16),
                        s
                    )
                })
                .map_err(|e| Error::new(std::io::ErrorKind::Other, e.cause)),
            Command::Save(save) => {
                // Check if the user, password and url are provided
                if save.user.is_none() || save.password.is_none() || save.url.is_none() {
                    let mut missing = String::new();
                    if save.user.is_none() {
                        missing.push_str("user ");
                    }
                    if save.password.is_none() {
                        missing.push_str("password ");
                    }
                    if save.url.is_none() {
                        missing.push_str("url ");
                    }

                    let splitted = missing.split_whitespace().collect::<Vec<&str>>();
                    let formatted = splitted[..splitted.len() - 1].join(", ");

                    Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("Missing parameters: {}", formatted),
                    ))
                } else {
                    // TODO: Implement - here we should save the password in whatever way we want to
                    Ok(format!(
                        "User: {}, password: {}, url: {}\n saved successfully\n",
                        save.user.as_ref().unwrap(),
                        save.password.as_ref().unwrap(),
                        save.url.as_ref().unwrap()
                    ))
                }
            }
        }
    }
}
