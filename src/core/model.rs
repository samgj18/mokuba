pub mod error {
    use std::fmt::{Debug, Display, Formatter, Result};

    fn error_code_description(code: &ErrorCode) -> &str {
        match code {
            ErrorCode::LengthMustBeGreaterThanZero => "Length must be greater than zero",
            ErrorCode::UnableToConvertNumberToChar => "Unable to convert number to char",
            ErrorCode::UnableToReadInput => "Unable to read input",
            ErrorCode::UnableToWriteOutput => "Unable to write input",
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum ErrorCode {
        LengthMustBeGreaterThanZero,
        UnableToConvertNumberToChar,
        UnableToReadInput,
        UnableToWriteOutput,
    }

    impl Display for ErrorCode {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                ErrorCode::LengthMustBeGreaterThanZero => {
                    write!(f, "Length must be greater than 0")
                }
                ErrorCode::UnableToConvertNumberToChar => {
                    write!(f, "Unable to convert number to char")
                }
                ErrorCode::UnableToReadInput => write!(f, "Unable to read input"),
                ErrorCode::UnableToWriteOutput => write!(f, "Unable to write input"),
            }
        }
    }

    /// Generic error type for all errors that can occur in this crate.
    trait MokubaError<T: Display + Debug> {
        fn description(&self) -> &str;
        fn cause(&self) -> Option<&str>;
    }

    // GenError is a custom error type for the password generator.
    #[derive(Debug)]
    pub struct GenError {
        pub code: ErrorCode,
        pub cause: Option<String>,
    }

    impl Display for GenError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}", self.code)
        }
    }

    impl MokubaError<GenError> for GenError {
        fn description(&self) -> &str {
            error_code_description(&self.code)
        }

        fn cause(&self) -> Option<&str> {
            self.cause.as_deref()
        }
    }

    impl GenError {
        pub fn new(code: ErrorCode, cause: Option<&str>) -> GenError {
            GenError {
                code,
                cause: cause.map(|s| s.to_string()),
            }
        }
    }

    #[derive(Debug)]
    pub struct GetInputError {
        pub code: ErrorCode,
        pub cause: Option<String>,
    }

    impl Display for GetInputError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}", self.code)
        }
    }

    impl MokubaError<GetInputError> for GetInputError {
        fn description(&self) -> &str {
            error_code_description(&self.code)
        }

        fn cause(&self) -> Option<&str> {
            self.cause.as_deref()
        }
    }

    impl GetInputError {
        pub fn new(cause: &str, code: ErrorCode) -> GetInputError {
            GetInputError {
                code,
                cause: Some(cause.to_string()),
            }
        }
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
