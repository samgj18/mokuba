use model::error::ErrorCode::{LengthMustBeGreaterThanZero, UnableToConvertNumberToChar};
use model::{error::GenError, params::PassParams};

pub mod codec;
pub mod model;
pub mod mstd;
pub mod parser;

/**

## Password Generator

### Examples
```
use mcore::{gen, model::params::PassParams};
let password = gen(PassParams { length: 10 });
```

This function will return an error if the length is less than 1 or if the acc seed is different than an empty string.
*/
pub fn gen(params: PassParams) -> Result<String, GenError> {
    gen_with_seed(params, "")
}

/**

### Arguments

* `params` - `PassParams` struct containing the length of the password to generate
* `seed` - `String` containing the seed to use for the password generation

### Returns
This function will return an error if the length is less than 1 or if the acc seed is different than an empty string.

*/
fn gen_with_seed(params: PassParams, acc: &str) -> Result<String, GenError> {
    use rand::{thread_rng, Rng};
    use std::char::from_u32;

    if params.length == 0 {
        if acc.is_empty() {
            return Err(GenError::new(
                LengthMustBeGreaterThanZero,
                Some("Please choose a length greater than 0"),
            ));
        };
        return Ok(acc.to_string());
    }

    let codepoint = thread_rng().gen_range(48..122);

    match from_u32(codepoint) {
        Some(value) => gen_with_seed(
            PassParams::new(params.length - 1),
            &format!("{}{}", acc, value),
        ),
        None => Err(GenError::new(
            UnableToConvertNumberToChar,
            Some(&format!(
                "Unable to convert codepoint:{} to char",
                codepoint
            )),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        gen_with_seed, model::error::ErrorCode::LengthMustBeGreaterThanZero,
        model::params::PassParams,
    };

    #[test]
    fn produces_a_password_of_10_digits() {
        let result = gen_with_seed(PassParams { length: 10 }, "");
        assert_eq!(result.unwrap().len(), 10);
    }

    #[test]
    fn produces_a_password_of_20_digits() {
        let result = gen_with_seed(PassParams { length: 20 }, "");
        assert_eq!(result.unwrap().len(), 20);
    }

    #[test]
    fn produces_a_password_of_30_digits() {
        let result = gen_with_seed(PassParams { length: 30 }, "");
        assert_eq!(result.unwrap().len(), 30);
    }

    #[test]
    fn produces_an_error_when_length_is_less_than_1() {
        let result = gen_with_seed(PassParams { length: 0 }, "");
        assert!(result.unwrap_err().code == LengthMustBeGreaterThanZero);
    }

    #[test]
    fn produces_a_password_prepended_with_a_seed() {
        let test_seed = "test_seed";
        let result = gen_with_seed(PassParams { length: 15 }, test_seed);
        assert!(result.unwrap().len() == 15 + test_seed.len());
    }
}
