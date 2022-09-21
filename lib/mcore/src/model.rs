pub mod error {
    use mmacro::{ConstructorM, DisplayM, ErrorM, VariantM};
    use std::fmt::Debug;
    use std::io::Error;

    #[derive(Debug, PartialEq, Eq, DisplayM)]
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

    // GenError is a custom error type for the password generator.
    #[derive(Debug, ErrorM, DisplayM, ConstructorM)]
    pub struct GenError {
        pub code: ErrorCode,
        pub cause: Option<String>,
    }

    // InputError is a custom error type for the input parser.
    #[derive(Debug, ErrorM, DisplayM, ConstructorM)]
    pub struct GetInputError {
        pub code: ErrorCode,
        pub cause: Option<String>,
    }

    // DecodeError is a custom error type for the input parser.
    #[derive(Debug, ErrorM, DisplayM, ConstructorM)]
    pub struct DecodeError {
        pub code: ErrorCode,
        pub cause: Option<String>,
    }

    // ParseError is a custom error type for the input parser.
    #[derive(Debug, ErrorM, DisplayM, ConstructorM)]
    pub struct ParseError {
        pub code: ErrorCode,
        pub cause: Option<String>,
    }
}

pub mod params {
    use mmacro::ConstructorM;

    #[derive(Debug, ConstructorM)]
    pub struct PassParams {
        pub length: u32,
    }

    impl Default for PassParams {
        fn default() -> Self {
            PassParams { length: 10 }
        }
    }
}
