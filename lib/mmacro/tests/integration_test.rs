use mmacro::*;
use std::fmt::{Debug, Display};
use std::io::{Error, ErrorKind};

#[test]
fn test_derive_enum_variants() {
    #[derive(Debug, VariantM)]
    enum Example {
        LengthMustBeGreaterThanZero,
        UnableToConvertNumberToChar,
        UnableToReadInput,
        UnableToWriteOutput,
        UnableToParseInput,
        UnableToEncodeT,
        UnableToDecodeT,
    }

    trait MokubaError<T>
    where
        T: Display + Debug,
    {
        fn description(&self) -> &str;
        fn cause(&self) -> Option<&str>;
    }

    assert_eq!(
        Example::LengthMustBeGreaterThanZero.variant(),
        "LengthMustBeGreaterThanZero"
    );
    assert_eq!(
        Example::UnableToConvertNumberToChar.variant(),
        "UnableToConvertNumberToChar"
    );
    assert_eq!(Example::UnableToReadInput.variant(), "UnableToReadInput");
    assert_eq!(
        Example::UnableToWriteOutput.variant(),
        "UnableToWriteOutput"
    );
    assert_eq!(Example::UnableToParseInput.variant(), "UnableToParseInput");
    assert_eq!(Example::UnableToEncodeT.variant(), "UnableToEncodeT");
    assert_eq!(Example::UnableToDecodeT.variant(), "UnableToDecodeT");
}

#[test]
fn test_derive_parser() {
    #[derive(ParserM)]
    enum Example {
        LengthMustBeGreaterThanZero,
        UnableToConvertNumberToChar,
        UnableToReadInput,
        UnableToWriteOutput,
        UnableToParseInput,
        UnableToEncodeT,
        UnableToDecodeT,
    }

    assert!(Example::parse("LengthMustBeGreaterThanZero").is_err());
    assert!(Example::parse("LengthMustBeGreaterThanZero".to_lowercase().as_str()).is_ok());

    assert!(Example::parse("UnableToConvertNumberToChar").is_err());
    assert!(Example::parse("UnableToConvertNumberToChar".to_lowercase().as_str()).is_ok());

    assert!(Example::parse("UnableToReadInput").is_err());
    assert!(Example::parse("UnableToReadInput".to_lowercase().as_str()).is_ok());

    assert!(Example::parse("UnableToWriteOutput").is_err());
    assert!(Example::parse("UnableToWriteOutput".to_lowercase().as_str()).is_ok());

    assert!(Example::parse("UnableToParseInput").is_err());
    assert!(Example::parse("UnableToParseInput".to_lowercase().as_str()).is_ok());

    assert!(Example::parse("UnableToEncodeT").is_err());
    assert!(Example::parse("UnableToEncodeT".to_lowercase().as_str()).is_ok());
}

#[test]
fn test_derive_display() {
    #[derive(Debug, PartialEq, Eq, DisplayM)]
    pub enum Command {
        Help,
        Version,
    }

    assert_eq!(Command::Help.to_string(), "Help");
    assert_eq!(Command::Version.to_string(), "Version");
}

#[test]
fn test_derive_constructor() {
    #[derive(ConstructorM)]
    pub struct Command {
        pub name: String,
        pub description: String,
    }

    let command = Command::new("help".to_string(), "Help".to_string());
    assert_eq!(command.name, "help");
    assert_eq!(command.description, "Help");
}

#[test]
fn test_derive_std_error() {
    #[derive(Debug, PartialEq, Eq, DisplayM)]
    pub enum Example {
        LengthMustBeGreaterThanZero,
        UnableToConvertNumberToChar,
        UnableToReadInput,
        UnableToWriteOutput,
        UnableToParseInput,
        UnableToEncodeT,
        UnableToDecodeT,
    }
    #[derive(Debug, PartialEq, Eq, DisplayM, ErrorM)]
    pub struct SuperComplicatedError {
        pub message: String,
        pub cause: Option<String>,
        pub e_type: Example,
        pub line: usize,
    }

    impl SuperComplicatedError {
        pub fn new(message: String, cause: Option<String>, e_type: Example, line: usize) -> Self {
            Self {
                message,
                cause,
                e_type,
                line,
            }
        }
    }

    let error = SuperComplicatedError::new(
        "Unable to read input".to_string(),
        Some("Unable to read input".to_string()),
        Example::UnableToReadInput,
        1,
    );

    let std_error = Error::from(error);

    assert_eq!(std_error.kind(), ErrorKind::Other);
    assert_eq!(std_error.to_string(), "message: \"Unable to read input\" cause: Some(\"Unable to read input\") e_type: UnableToReadInput line: 1 ");
}
