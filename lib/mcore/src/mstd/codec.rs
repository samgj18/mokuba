use std::{
    convert::{TryFrom, TryInto},
    fmt::{Debug, Display},
    str::FromStr,
};

use super::error::{DecodeError, ErrorCode::UnableToDecodeT};

pub trait Codec<A> {
    /**
    Encodes a value of type `A` into a `String`

    # Examples

    ```
    use mcore::mstd::codec::Codec;

    let str = "hello world".to_string();
    assert_eq!(str.encode(), "hello world");
    assert_eq!(Some("hello world".to_string()).unwrap(), str);
    assert!(String::is(str));
    ```
    */
    fn encode(&self) -> String;

    /**
    Decodes a value of type `A` from a `String`

    # Examples

    ```
    use mcore::mstd::codec::Codec;

    assert!(bool::decode(Some("hello world")).is_err());
    assert!(!bool::is("hello world".to_string()));
    ```
    */
    fn decode(s: Option<&str>) -> Result<A, DecodeError>;

    /**
    Checks if a value of type `A` is in fact the type `A`

    # Examples

    ```
    use mcore::mstd::codec::Codec;

    assert!(u64::is("42".to_string()));
    assert!(!u64::is("hello world".to_string()));
    ```
    */
    fn is(s: String) -> bool;

    fn _as<T>(&self, a: A) -> Result<T, DecodeError>
    where
        T: TryFrom<A>,
    {
        a.try_into().map_err(|_| -> DecodeError {
            DecodeError::new(UnableToDecodeT, "Unable to decode T".to_owned())
        })
    }
}

impl Codec<String> for String {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: Option<&str>) -> Result<String, DecodeError> {
        match s {
            Some(s) => match s.is_empty() {
                true => Err(DecodeError::new(
                    UnableToDecodeT,
                    format!("Unable to decode {} to String", s),
                )),
                false => Ok(s.to_string()),
            },
            None => Err(DecodeError::new(
                UnableToDecodeT,
                "Unable to decode because the String is empty".to_owned(),
            )),
        }
    }

    fn is(_s: String) -> bool {
        true
    }
}

impl Codec<u32> for u32 {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: Option<&str>) -> Result<u32, DecodeError> {
        match s {
            Some(s) => s.parse::<u32>().map_err(|e| -> DecodeError {
                DecodeError::new(
                    UnableToDecodeT,
                    format!("Unable to decode {} to u32: {}", s, e),
                )
            }),
            None => Err(DecodeError::new(
                UnableToDecodeT,
                "Unable to decode because the u32 is empty".to_owned(),
            )),
        }
    }

    fn is(s: String) -> bool {
        s.parse::<u32>().is_ok()
    }
}

impl Codec<u64> for u64 {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: Option<&str>) -> Result<u64, DecodeError> {
        match s {
            Some(s) => s.parse::<u64>().map_err(|e| -> DecodeError {
                DecodeError::new(
                    UnableToDecodeT,
                    format!("Unable to decode {} to u64: {}", s, e),
                )
            }),
            None => Err(DecodeError::new(
                UnableToDecodeT,
                "Unable to decode because the u64 is empty".to_owned(),
            )),
        }
    }

    fn is(s: String) -> bool {
        s.parse::<u64>().is_ok()
    }
}

impl Codec<bool> for bool {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: Option<&str>) -> Result<bool, DecodeError> {
        match s {
            Some(s) => s.parse::<bool>().map_err(|e| -> DecodeError {
                DecodeError::new(
                    UnableToDecodeT,
                    format!("Unable to decode {} to bool: {}", s, e),
                )
            }),
            None => Err(DecodeError::new(
                UnableToDecodeT,
                "Unable to decode because the bool is empty".to_owned(),
            )),
        }
    }

    fn is(s: String) -> bool {
        s.parse::<bool>().is_ok()
    }
}

impl Codec<char> for char {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: Option<&str>) -> Result<char, DecodeError> {
        match s {
            Some(s) => {
                if s.len() == 1 {
                    s.chars().next().ok_or_else(|| -> DecodeError {
                        DecodeError::new(UnableToDecodeT, format!("Unable to decode {} to char", s))
                    })
                } else {
                    Err(DecodeError::new(
                        UnableToDecodeT,
                        format!("Unable to decode {} to char", s),
                    ))
                }
            }
            None => Err(DecodeError::new(
                UnableToDecodeT,
                "Unable to decode because the char is empty".to_owned(),
            )),
        }
    }

    fn is(s: String) -> bool {
        s.chars().count() == 1
    }
}

impl<A: Display + FromStr + Debug> Codec<Vec<A>> for Vec<A> {
    fn encode(&self) -> String {
        self.iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn decode(s: Option<&str>) -> Result<Vec<A>, DecodeError> {
        match s {
            Some(s) => s
                .split_whitespace()
                .map(|s| -> Result<A, DecodeError> {
                    s.parse::<A>().map_err(|_| -> DecodeError {
                        DecodeError::new(
                            UnableToDecodeT,
                            format!("Unable to decode {} to Vec<A>", s),
                        )
                    })
                })
                .collect::<Result<Vec<A>, DecodeError>>(),

            None => Err(DecodeError::new(
                UnableToDecodeT,
                "Unable to decode because the Vec<A> is empty".to_owned(),
            )),
        }
    }

    fn is(s: String) -> bool {
        s.split_whitespace()
            .map(|s| -> Result<A, DecodeError> {
                s.parse::<A>().map_err(|_| -> DecodeError {
                    DecodeError::new(UnableToDecodeT, "Unable to decode T".to_owned())
                })
            })
            .collect::<Result<Vec<A>, DecodeError>>()
            .is_ok()
    }
}

impl<A: Display + FromStr> Codec<Option<A>> for Option<A> {
    fn encode(&self) -> String {
        match self {
            Some(a) => a.to_string(),
            None => "".to_string(),
        }
    }

    fn decode(s: Option<&str>) -> Result<Option<A>, DecodeError> {
        match s {
            Some(s) => s.parse::<A>().map(|a| Some(a)).map_err(|_| -> DecodeError {
                DecodeError::new(UnableToDecodeT, "Unable to decode T".to_owned())
            }),
            None => Ok(None),
        }
    }

    fn is(s: String) -> bool {
        s.parse::<A>().is_ok()
    }
}

impl Codec<i32> for i32 {
    fn encode(&self) -> String {
        self.to_string()
    }

    fn decode(s: Option<&str>) -> Result<i32, DecodeError> {
        match s {
            Some(s) => s.parse::<i32>().map_err(|_| -> DecodeError {
                DecodeError::new(UnableToDecodeT, "Unable to decode T".to_owned())
            }),
            None => Err(DecodeError::new(
                UnableToDecodeT,
                "Unable to decode T".to_owned(),
            )),
        }
    }

    fn is(s: String) -> bool {
        s.parse::<i32>().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_should_succeed() {
        let str = "hello world".to_string();
        assert_eq!(str.encode(), "hello world");
        assert_eq!(Some("hello world".to_string()).unwrap(), str);
        assert!(String::is(str));
    }

    #[test]
    fn test_u32_should_succeed() {
        let u = 42u32;
        assert_eq!(u.encode(), "42");
        assert_eq!(u32::decode(Some("42")).unwrap(), u);
        assert!(u32::is("42".to_string()))
    }

    #[test]
    fn test_u32_should_fail_when_not_a_number() {
        assert!(u32::decode(Some("hello world")).is_err());
        assert!(!u32::is("hello world".to_string()));
    }

    #[test]
    fn test_u64_should_succeed() {
        let u = 42u64;
        assert_eq!(u.encode(), "42");
        assert_eq!(u64::decode(Some("42")).unwrap(), u);
        assert!(u64::is("42".to_string()));
    }

    #[test]
    fn test_u64_should_fail_when_not_a_number() {
        assert!(u64::decode(Some("hello world")).is_err());
        assert!(!u64::is("hello world".to_string()));
    }

    #[test]
    fn test_bool_should_succeed() {
        let b = true;
        assert_eq!(b.encode(), "true");
        assert_eq!(bool::decode(Some("true")).unwrap(), b);
        assert!(bool::is("true".to_string()));
    }

    #[test]
    fn test_bool_should_fail_when_not_a_bool() {
        assert!(bool::decode(Some("hello world")).is_err());
        assert!(!bool::is("hello world".to_string()));
    }

    #[test]
    fn test_char_should_succeed() {
        let c = 'a';
        assert_eq!(c.encode(), "a");
        assert_eq!(char::decode(Some("a")).unwrap(), c);
        assert!(char::is("a".to_string()));
    }

    #[test]
    fn test_char_should_fail_when_not_a_char() {
        assert!(char::decode(Some("hello world")).is_err());
        assert!(!char::is("hello world".to_string()));
    }

    #[test]
    fn test_vec_should_succeed() {
        let v = vec![1, 2, 3];
        assert_eq!(v.encode(), "1 2 3");
        assert_eq!(Vec::<u32>::decode(Some("1 2 3")).unwrap(), v);
        assert!(Vec::<u32>::is("1 2 3".to_string()));
    }

    #[test]
    fn test_vec_should_fail_when_not_a_vec() {
        assert!(Vec::<u32>::decode(Some("hello world")).is_err());
        assert!(!Vec::<u32>::is("hello world".to_string()));
    }

    #[test]
    fn test_vec_of_strings_should_succeed() {
        let v = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(v.encode(), "hello world");
        assert_eq!(Vec::<String>::decode(Some("hello world")).unwrap(), v);
        assert!(Vec::<String>::is("hello world".to_string()));
    }

    #[test]
    fn test_vec_of_strings_should_fail_when_not_a_vec_of_u32() {
        assert!(Vec::<u32>::decode(Some("hello world 42")).is_err());
    }

    #[test]
    fn test_something() {
        let asdas = "";
        assert!(asdas.is_empty());
    }
}
