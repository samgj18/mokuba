use std::collections::HashSet;

use super::codec::Codec;
use crate::gen;

use super::{
    cmd::{Argument, Execute, Parse, HELP},
    param::GenerateParams,
};

// Available commands
pub struct Generate;
pub struct Help;

impl Execute<GenerateParams> for Generate {
    fn execute(&self, params: Option<GenerateParams>) -> Result<String, String> {
        match params {
            Some(p) => gen(p).map_err(|e| e.cause),
            None => gen(GenerateParams::new(16)).map_err(|e| e.cause),
        }
    }
}

impl Argument for Generate {
    fn short(&self) -> Option<char> {
        Some('g')
    }

    fn argument(&self) -> String {
        "generate".to_string()
    }

    fn is_valid_flag(key: &str) -> bool {
        HashSet::from(["-p", "--password", "-w", "--website", "-u", "--username"]).contains(key)
    }

    fn description(&self) -> String {
        "Generate a password".to_string()
    }
}

impl Parse<GenerateParams> for Generate {
    fn parse(s: Vec<&str>) -> Result<GenerateParams, String> {
        // parsing first argument for fail fast approach
        s.first().map_or_else(
            || Err("No arguments provided".to_string()),
            |s| {
                let mut iter = s.split('=');
                let key = iter.next();
                let value = iter.next();

                match (key, value) {
                    (Some(v1), Some(v2)) => {
                        if !Self::is_valid_flag(v1) {
                            return Err(format!("Invalid flag: {}", v1));
                        }

                        u32::decode(Some(v2))
                            .map_err(|e| e.cause)
                            .map(GenerateParams::new)
                    }
                    (Some(k), _) => Err(format!("Unknown key: {}", k)),
                    (None, _) => Err("No key provided".to_string()),
                }
            },
        )
    }
}

impl Execute<()> for Help {
    fn execute(&self, _: Option<()>) -> Result<String, String> {
        Ok(HELP.to_owned())
    }
}

impl Argument for Help {
    fn short(&self) -> Option<char> {
        Some('h')
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
    fn parse(_: Vec<&str>) -> Result<(), String> {
        Ok(())
    }
}
