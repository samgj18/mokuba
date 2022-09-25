use crate::mstd::error::ErrorCode::UnableToParseInputToT;
use std::collections::HashSet;

use super::{cmd::Input, codec::Codec, error::GetInputError};
use crate::gen;

use super::{
    cmd::{Argument, Execute, Parse, HELP},
    param::GenerateParams,
};

// Available commands
#[derive(Debug, Copy, Clone)]
pub struct Generate;

impl Execute<GenerateParams> for Generate {
    fn execute(&self, params: Option<GenerateParams>) -> Result<String, String> {
        match params {
            Some(p) => gen(p).map_err(|e| e.cause),
            None => gen(GenerateParams::new(16, None)).map_err(|e| e.cause),
        }
    }
}

impl Argument for Generate {
    fn short(&self) -> char {
        'g'
    }

    fn argument(&self) -> String {
        "generate".to_string()
    }

    fn is_valid_flag(key: &str) -> bool {
        HashSet::from(["-p", "--password", "-u", "--username"]).contains(key)
    }

    fn description(&self) -> String {
        format!(
            "{} {}: Generate a password with the given length and username. 
            If no length is given, the default length is 16. 
            If no username is given, only a password is generated. 
            If a username is given, a password and a username are generated.
            
            Usage: generate --password Optional<{{}} --username <<{{}}>>>",
            self.short(),
            self.argument()
        )
    }
}

impl Parse<GenerateParams> for Generate {
    fn parse(&self, input: &Input) -> Result<GenerateParams, String> {
        match validate(self, input) {
            Ok(_) => (),
            Err(e) => return Err(e.cause),
        }

        let option_password = input
            .params
            .get("-p")
            .or_else(|| input.params.get("--password"));

        let option_username = input
            .params
            .get("-u")
            .or_else(|| input.params.get("--username"));

        let password = u32::decode(option_password.map(|s| s.as_str()));
        let username = String::decode(option_username.map(|s| s.as_str()));

        match (password, username) {
            (Ok(p), Ok(u)) => Ok(GenerateParams::new(p, Some(u))),
            (Err(_), _) => Ok(GenerateParams::new(16, None)),
            (_, Err(e)) => Err(e.cause),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Help;

impl Execute<()> for Help {
    fn execute(&self, _: Option<()>) -> Result<String, String> {
        Ok(HELP.to_owned())
    }
}

impl Argument for Help {
    fn short(&self) -> char {
        'h'
    }

    fn argument(&self) -> String {
        "help".to_string()
    }

    fn is_valid_flag(_key: &str) -> bool {
        true
    }

    fn description(&self) -> String {
        "Show this help message".to_string()
    }
}

impl Parse<()> for Help {
    fn parse(&self, _: &Input) -> Result<(), String> {
        Ok(())
    }
}

fn validate<C, P>(command: &C, input: &Input) -> Result<(), GetInputError>
where
    C: Parse<P> + Execute<P> + Argument,
{
    if input.arg.is_empty() {
        return Err(GetInputError::new(
            UnableToParseInputToT,
            "No argument provided".to_string(),
        ));
    }

    if input.arg != command.argument() {
        return Err(GetInputError::new(
            UnableToParseInputToT,
            format!("Invalid argument: {}", input.arg),
        ));
    }

    // check if an invalid flag is provided and return an error with the invalid flag
    let invalid_flag = input
        .params
        .keys()
        .find(|key| !C::is_valid_flag(key.as_str()));

    if let Some(flag) = invalid_flag {
        return Err(GetInputError::new(
            UnableToParseInputToT,
            format!("Invalid flag provided: {}", flag),
        ));
    }

    Ok(())
}
