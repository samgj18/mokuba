use crate::core::model::{GenError, Params};

/**

### Arguments

* `params` - `Params` struct containing the length of the password to generate
* `seed` - `String` containing the seed to use for the password generation

### Returns
This function will return an error if the length is less than 1 or if the acc seed is different than an empty string.

*/
pub fn gen_with_seed(params: Params, acc: &str) -> Result<String, GenError> {
    use rand::{thread_rng, Rng};
    use std::char::from_u32;

    if params.length == 0 {
        if acc.is_empty() {
            return Err(GenError::new("Length must be greater than 0"));
        };
        return Ok(acc.to_string());
    }

    let codepoint = thread_rng().gen_range(48..122);

    match from_u32(codepoint) {
        Some(value) => gen_with_seed(Params::new(params.length - 1), &format!("{}{}", acc, value)),
        None => Err(GenError::new("Unable to convert number to char")),
    }
}

mod tests {
    #[cfg(test)]
    use crate::core::{gen_with_seed, model::Params};

    #[test]
    fn produces_a_password_of_10_digits() {
        let result = gen_with_seed(Params { length: 10 }, "");
        assert_eq!(result.unwrap().len(), 10);
    }

    #[test]
    fn produces_a_password_of_20_digits() {
        let result = gen_with_seed(Params { length: 20 }, "");
        assert_eq!(result.unwrap().len(), 20);
    }

    #[test]
    fn produces_a_password_of_30_digits() {
        let result = gen_with_seed(Params { length: 30 }, "");
        assert_eq!(result.unwrap().len(), 30);
    }

    #[test]
    fn produces_an_error_when_length_is_less_than_1() {
        let result = gen_with_seed(Params { length: 0 }, "");
        assert!(result.unwrap_err().reason == "Length must be greater than 0");
    }
}

pub mod model;
