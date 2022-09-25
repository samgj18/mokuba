pub mod algebra;
pub mod mstd;
pub mod syntax;

use mstd::{
    error::{
        ErrorCode::{LengthMustBeGreaterThanZero, UnableToConvertNumberToChar},
        GenError,
    },
    param::GenerateParams,
};
/**

## Password Generator

### Examples
```
use mcore::{gen, mstd::param::GenerateParams};
let password = gen(GenerateParams { length: 10, username: None });
```

This function will return an error if the length is less than 1 or if the acc seed is different than an empty string.
*/
pub fn gen(params: GenerateParams) -> Result<String, GenError> {
    gen_with_seed(params, "")
}

/**

### Arguments

* `params` - `GenerateParams` struct containing the length of the password to generate
* `seed` - `String` containing the seed to use for the password generation

### Returns
This function will return an error if the length is less than 1 or if the acc seed is different than an empty string.

*/
fn gen_with_seed(params: GenerateParams, acc: &str) -> Result<String, GenError> {
    use rand::{thread_rng, Rng};
    use std::char::from_u32;

    if params.length == 0 {
        if acc.is_empty() {
            return Err(GenError::new(
                LengthMustBeGreaterThanZero,
                "Please choose a length greater than 0".to_owned(),
            ));
        };
        return Ok(acc.to_string());
    }

    let codepoint = thread_rng().gen_range(48..122);

    match from_u32(codepoint) {
        Some(value) => gen_with_seed(
            GenerateParams::new(params.length - 1, None),
            &format!("{}{}", acc, value),
        ),
        None => Err(GenError::new(
            UnableToConvertNumberToChar,
            format!("Unable to convert codepoint:{} to char", codepoint),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{gen_with_seed, GenerateParams, LengthMustBeGreaterThanZero};

    #[test]
    fn produces_a_password_of_10_digits() {
        let result = gen_with_seed(GenerateParams::new(10, None), "");
        assert_eq!(result.unwrap().len(), 10);
    }

    #[test]
    fn produces_a_password_of_20_digits() {
        let result = gen_with_seed(GenerateParams::new(20, None), "");
        assert_eq!(result.unwrap().len(), 20);
    }

    #[test]
    fn produces_a_password_of_30_digits() {
        let result = gen_with_seed(GenerateParams::new(30, None), "");
        assert_eq!(result.unwrap().len(), 30);
    }

    #[test]
    fn produces_an_error_when_length_is_less_than_1() {
        let result = gen_with_seed(GenerateParams::new(0, None), "");
        assert!(result.unwrap_err().code == LengthMustBeGreaterThanZero);
    }

    #[test]
    fn produces_a_password_prepended_with_a_seed() {
        let test_seed = "test_seed";
        let result = gen_with_seed(GenerateParams::new(15, None), test_seed);
        assert!(result.unwrap().len() == 15 + test_seed.len());
    }
}
