use crate::model::error::ParseError;

trait Parser {
    fn parse(&self, data: &str) -> Result<String, ParseError>;
}
