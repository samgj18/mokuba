use mmacro::ConstructorM;

use std::collections::HashMap;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BIN_NAME: &str = env!("CARGO_PKG_NAME");
pub static HELP: &str = concat!(
    "Usage: {} [OPTION]...\n",
    env!("CARGO_PKG_NAME"),
    " [command] [options]\n",
);

#[derive(Debug, ConstructorM)]
pub struct Input {
    pub arg: String,
    pub params: HashMap<String, String>,
}

pub trait Execute<P> {
    fn execute(&self, params: Option<P>) -> Result<String, String>;
}

pub trait Argument {
    fn short(&self) -> char;
    fn argument(&self) -> String;

    fn description(&self) -> String;

    fn is_valid_flag(key: &str) -> bool;
}

pub trait Parse<O> {
    fn parse(&self, s: &Input) -> Result<O, String>;
}
