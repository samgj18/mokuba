pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BIN_NAME: &str = env!("CARGO_PKG_NAME");
pub static HELP: &str = concat!(
    "Usage: {} [OPTION]...\n",
    env!("CARGO_PKG_NAME"),
    " [command] [options]\n",
);

pub trait Execute<P> {
    fn execute(&self, params: Option<P>) -> Result<String, String>;
}

pub trait Argument {
    fn short(&self) -> Option<char>;
    fn argument(&self) -> String;

    fn is_valid_flag(key: &str) -> bool;
    fn description(&self) -> String;
}

pub trait Parse<O> {
    fn parse(s: Vec<&str>) -> Result<O, String>;
}

pub trait Command<C, P>
where
    C: Parse<P> + Execute<P> + Argument,
{
}

impl<C, P> Command<C, P> for C where C: Execute<P> + Argument + Parse<P> {}
