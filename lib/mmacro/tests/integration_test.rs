use mmacro::*;
use std::fmt::Debug;
use std::fmt::Display;

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
        Example::LengthMustBeGreaterThanZero.description_m(),
        "LengthMustBeGreaterThanZero"
    );
    assert_eq!(
        Example::UnableToConvertNumberToChar.description_m(),
        "UnableToConvertNumberToChar"
    );
    assert_eq!(
        Example::UnableToReadInput.description_m(),
        "UnableToReadInput"
    );
    assert_eq!(
        Example::UnableToWriteOutput.description_m(),
        "UnableToWriteOutput"
    );
    assert_eq!(
        Example::UnableToParseInput.description_m(),
        "UnableToParseInput"
    );
    assert_eq!(Example::UnableToEncodeT.description_m(), "UnableToEncodeT");
    assert_eq!(Example::UnableToDecodeT.description_m(), "UnableToDecodeT");
}
