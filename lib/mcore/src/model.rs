pub mod error {
    use mmacro::{DisplayM, MokubaErrorM, VariantM};
    use std::fmt::{Debug, Display, Formatter, Result};

    #[derive(Debug, PartialEq, Eq, DisplayM, VariantM)]
    pub enum ErrorCode {
        LengthMustBeGreaterThanZero,
        UnableToConvertNumberToChar,
        UnableToReadInput,
        UnableToWriteOutput,
        UnableToParseInput,
        UnableToEncodeT,
        UnableToDecodeT,
        UnableToParseInputToT,
    }

    /// Generic error type for all errors that can occur in this crate.
    trait MokubaError<T: Display + Debug> {
        fn description(&self) -> &str;
        fn cause(&self) -> Option<&str>;
    }

    // GenError is a custom error type for the password generator.
    #[derive(Debug, MokubaErrorM)]
    pub struct GenError {
        pub code: ErrorCode,
        pub cause: Option<String>,
    }

    // InputError is a custom error type for the input parser.
    #[derive(Debug, MokubaErrorM)]
    pub struct GetInputError {
        pub code: ErrorCode,
        pub cause: Option<String>,
    }

    // DecodeError is a custom error type for the input parser.
    #[derive(Debug, MokubaErrorM)]
    pub struct DecodeError {
        pub code: ErrorCode,
        pub cause: Option<String>,
    }

    // ParseError is a custom error type for the input parser.
    #[derive(Debug, MokubaErrorM)]
    pub struct ParseError {
        pub code: ErrorCode,
        pub cause: Option<String>,
    }
}

pub mod params {
    #[derive(Debug)]
    pub struct PassParams {
        pub length: u32,
    }

    impl PassParams {
        pub fn new(length: u32) -> PassParams {
            PassParams { length }
        }
    }

    impl Default for PassParams {
        fn default() -> Self {
            PassParams { length: 10 }
        }
    }
}
