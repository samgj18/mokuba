use mmacro::{ConstructorM, DisplayM, ErrorM, VariantM};
use std::{fmt::Debug, io::Error};

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

// GenError is a custom error type for the password generator.
#[derive(Debug, ErrorM, DisplayM, ConstructorM)]
pub struct GenError {
    pub code: ErrorCode,
    pub cause: String,
}

// InputError is a custom error type for the input parser.
#[derive(Debug, ErrorM, DisplayM, ConstructorM)]
pub struct GetInputError {
    pub code: ErrorCode,
    pub cause: String,
}

// DecodeError is a custom error type for the input parser.
#[derive(Debug, ErrorM, DisplayM, ConstructorM)]
pub struct DecodeError {
    pub code: ErrorCode,
    pub cause: String,
}

// ParseError is a custom error type for the input parser.
#[derive(Debug, ErrorM, DisplayM, ConstructorM)]
pub struct ParseError {
    pub code: ErrorCode,
    pub cause: String,
}
