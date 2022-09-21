use crate::model::error::{DecodeError, ErrorCode::UnableToDecodeT};
use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    str::FromStr,
};

pub trait Codec<A> {
    fn encode(&self) -> String;
    fn decode(s: String) -> Result<A, DecodeError>;
    fn is(s: String) -> bool;

    // Create a function call _as with a generic type T which is a subtype of A and return the input as a T.
    fn _as<T>(&self, a: A) -> Result<T, DecodeError>
    where
        T: TryFrom<A>,
    {
        a.try_into().map_err(|_| -> DecodeError {
            DecodeError::new(UnableToDecodeT, Some("Unable to decode T".to_owned()))
        })
    }
}

impl Codec<String> for String {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: String) -> Result<String, DecodeError> {
        Ok(s)
    }

    fn is(_s: String) -> bool {
        true
    }
}

impl Codec<u32> for u32 {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: String) -> Result<u32, DecodeError> {
        s.parse::<u32>().map_err(|_| -> DecodeError {
            DecodeError::new(UnableToDecodeT, Some("Unable to decode T".to_owned()))
        })
    }

    fn is(s: String) -> bool {
        s.parse::<u32>().is_ok()
    }
}

impl Codec<u64> for u64 {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: String) -> Result<u64, DecodeError> {
        s.parse::<u64>().map_err(|_| -> DecodeError {
            DecodeError::new(UnableToDecodeT, Some("Unable to decode T".to_owned()))
        })
    }

    fn is(s: String) -> bool {
        s.parse::<u64>().is_ok()
    }
}

impl Codec<bool> for bool {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: String) -> Result<bool, DecodeError> {
        s.parse::<bool>().map_err(|_| -> DecodeError {
            DecodeError::new(UnableToDecodeT, Some("Unable to decode T".to_owned()))
        })
    }

    fn is(s: String) -> bool {
        s.parse::<bool>().is_ok()
    }
}

impl Codec<char> for char {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: String) -> Result<char, DecodeError> {
        if s.len() == 1 {
            s.chars().next().ok_or_else(|| -> DecodeError {
                DecodeError::new(UnableToDecodeT, Some("Unable to decode T".to_owned()))
            })
        } else {
            Err(DecodeError::new(
                UnableToDecodeT,
                Some("Unable to decode T".to_owned()),
            ))
        }
    }

    fn is(s: String) -> bool {
        s.chars().count() == 1
    }
}

impl<A: Display + FromStr> Codec<Vec<A>> for Vec<A> {
    fn encode(&self) -> String {
        self.iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn decode(s: String) -> Result<Vec<A>, DecodeError> {
        s.split_whitespace()
            .map(|s| -> Result<A, DecodeError> {
                s.parse::<A>().map_err(|_| -> DecodeError {
                    DecodeError::new(UnableToDecodeT, Some("Unable to decode T".to_owned()))
                })
            })
            .collect::<Result<Vec<A>, DecodeError>>()
    }

    fn is(s: String) -> bool {
        s.split_whitespace()
            .map(|s| -> Result<A, DecodeError> {
                s.parse::<A>().map_err(|_| -> DecodeError {
                    DecodeError::new(UnableToDecodeT, Some("Unable to decode T".to_owned()))
                })
            })
            .collect::<Result<Vec<A>, DecodeError>>()
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_should_succeed() {
        let str = "hello world".to_string();
        assert_eq!(str.encode(), "hello world");
        assert_eq!(String::decode("hello world".to_string()).unwrap(), str);
        assert!(String::is(str));
    }

    #[test]
    fn test_u32_should_succeed() {
        let u = 42u32;
        assert_eq!(u.encode(), "42");
        assert_eq!(u32::decode("42".to_string()).unwrap(), u);
        assert!(u32::is("42".to_string()));
    }

    #[test]
    fn test_u32_should_fail_when_not_a_number() {
        assert!(u32::decode("hello world".to_string()).is_err());
        assert!(!u32::is("hello world".to_string()));
    }

    #[test]
    fn test_u64_should_succeed() {
        let u = 42u64;
        assert_eq!(u.encode(), "42");
        assert_eq!(u64::decode("42".to_string()).unwrap(), u);
        assert!(u64::is("42".to_string()));
    }

    #[test]
    fn test_u64_should_fail_when_not_a_number() {
        assert!(u64::decode("hello world".to_string()).is_err());
        assert!(!u64::is("hello world".to_string()));
    }

    #[test]
    fn test_bool_should_succeed() {
        let b = true;
        assert_eq!(b.encode(), "true");
        assert_eq!(bool::decode("true".to_string()).unwrap(), b);
        assert!(bool::is("true".to_string()));
    }

    #[test]
    fn test_bool_should_fail_when_not_a_bool() {
        assert!(bool::decode("hello world".to_string()).is_err());
        assert!(!bool::is("hello world".to_string()));
    }

    #[test]
    fn test_char_should_succeed() {
        let c = 'a';
        assert_eq!(c.encode(), "a");
        assert_eq!(char::decode("a".to_string()).unwrap(), c);
        assert!(char::is("a".to_string()));
    }

    #[test]
    fn test_char_should_fail_when_not_a_char() {
        assert!(char::decode("hello world".to_string()).is_err());
        assert!(!char::is("hello world".to_string()));
    }

    #[test]
    fn test_vec_should_succeed() {
        let v = vec![1, 2, 3];
        assert_eq!(v.encode(), "1 2 3");
        assert_eq!(Vec::<u32>::decode("1 2 3".to_string()).unwrap(), v);
        assert!(Vec::<u32>::is("1 2 3".to_string()));
    }

    #[test]
    fn test_vec_should_fail_when_not_a_vec() {
        assert!(Vec::<u32>::decode("hello world".to_string()).is_err());
        assert!(!Vec::<u32>::is("hello world".to_string()));
    }

    #[test]
    fn test_vec_of_strings_should_succeed() {
        let v = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(v.encode(), "hello world");
        assert_eq!(Vec::<String>::decode("hello world".to_string()).unwrap(), v);
        assert!(Vec::<String>::is("hello world".to_string()));
    }

    #[test]
    fn test_vec_of_strings_should_fail_when_not_a_vec_of_u32() {
        assert!(Vec::<u32>::decode("hello world 42".to_string()).is_err());
    }
}
