pub mod cmd;
pub mod cmds;
pub mod codec;
pub mod error;
pub mod param;

use std::io::{BufRead, Error, Write};

use self::cmd::{Execute, Input, Parse};
use self::cmds::Generate;
use self::error::ErrorCode::UnableToReadInput;
use self::error::GetInputError;

/**
Reads a line from stdin and returns it as a `Result<String, GetInputError>`

# Examples

```
use mcore::mstd::read_line_from;

let stdio = std::io::stdin();
let input = stdio.lock();

let answer = read_line_from(input);
println!("was: {}", answer.unwrap());
```
*/
pub fn read_line_from<R: BufRead>(mut reader: R) -> Result<String, GetInputError> {
    let mut input = String::new();
    reader.read_line(&mut input).map_err(|e| {
        GetInputError::new(
            UnableToReadInput,
            format!("Unable to read input properly with error: {}", e),
        )
    })?;
    Ok(input)
}

/**
Writes a line to stdout and returns it as a `Result<(), GetInputError>`

# Examples

```
use mcore::mstd::write_line_to;

let stdio = std::io::stdout();
let mut output = stdio.lock();

write_line_to(&mut output, "Hello World!").unwrap();
```
*/
pub fn write_line_to<W: Write>(mut writer: W, line: &str) -> Result<(), Error> {
    writer.write_all(line.as_bytes()).map_err(|e| {
        Error::new(
            std::io::ErrorKind::Other,
            format!("Unable to write to stdout: {}", e),
        )
    })
}

/**
Deserializes a command by splitting it and turning it into an `Input` type.

# Examples

```
use mcore::mstd::deserialize;
use std::collections::HashMap;

let input = "generate --password --url www.google.com --username test".to_string();
let command = deserialize(&input);
assert_eq!(command.as_ref().unwrap().arg, "generate");
assert_eq!(
    command.as_ref().unwrap().params.get("--password").unwrap(),
    ""
);
assert_eq!(
    command.as_ref().unwrap().params.get("--url").unwrap(),
    "www.google.com"
);
assert_eq!(command.unwrap().params.get("--username").unwrap(), "test");
```
*/
pub fn deserialize(args_vec: &str) -> Result<Input, String> {
    // this function should be able to take an input like: generate --password 1234 --url www.google.com --user test and
    // return an Input struct like this: Input { command: "generate", params: { "--password": "1234", "--user": "test" } }
    // check that the first argument is not empty since it is required

    let mut iter = args_vec.split_whitespace();
    let command = iter.next().unwrap_or_default();

    if command.is_empty() {
        return Err("No command was provided".to_string());
    }

    // generate --password --url www.google.com --username test
    let mut params = std::collections::HashMap::new();
    let mut key = String::new();
    let mut value = String::new();

    for arg in iter {
        if arg.starts_with("--") || arg.starts_with('-') {
            if !key.is_empty() {
                params.insert(key, value);
            }
            key = arg.to_string();
            value = String::new();
        } else {
            value = arg.to_string();
        }
    }

    if !key.is_empty() {
        params.insert(key, value);
    }

    Ok(Input {
        arg: command.to_string(),
        params,
    })
}

/**
This function is used to match the input to a command and execute it.

# Examples

```
use mcore::mstd::matcher;
use mcore::mstd::cmd::Input;
use std::collections::HashMap;

let input = Input {
        arg: "generate".to_string(),
        params: HashMap::from([
            ("--password".to_string(), "".to_string()),
            ("--username".to_string(), "test".to_string()),
        ]),
    };
let command = matcher(&input);

assert_eq!(command.unwrap().len(), 16);
```
*/
pub fn matcher(input: &Input) -> Result<String, String> {
    match input.arg.as_str() {
        "generate" => {
            let command = Generate;
            match command.parse(input) {
                Ok(params) => command.execute(Some(params)),
                Err(e) => Err(e),
            }
        }
        _ => Err(format!("Command {} not found", input.arg)),
    }
}

#[cfg(test)]
mod tests {
    use crate::mstd::read_line_from;

    #[test]
    fn test_read_line_from_in_memory_is_ok() {
        let input = b"I'm George";
        let answer = read_line_from(&input[..]);

        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), "I'm George");
    }

    #[test]
    fn test_read_line_from_in_memory_is_err_when_input_is_not_utf8() {
        let input = b"\x80";
        let answer = read_line_from(&input[..]);

        assert!(answer.is_err());
    }

    #[test]
    fn test_write_line_to_in_memory_is_ok() {
        let mut output = Vec::new();
        let answer = super::write_line_to(&mut output, "I'm George");

        assert!(answer.is_ok());
        assert_eq!(output, b"I'm George");
    }

    #[test]
    fn test_prepare_input_is_ok() {
        let input = "generate --password --url www.google.com --username test";
        let command = super::deserialize(input);

        assert!(command.is_ok());
        assert_eq!(command.as_ref().unwrap().arg, "generate");
        assert_eq!(
            command.as_ref().unwrap().params.get("--password").unwrap(),
            ""
        );
        assert_eq!(
            command.as_ref().unwrap().params.get("--url").unwrap(),
            "www.google.com"
        );
        assert_eq!(command.unwrap().params.get("--username").unwrap(), "test");
    }

    #[test]
    fn test_prepare_input_is_err_when_input_is_empty() {
        let input = "";
        let command = super::deserialize(input);

        assert!(command.is_err());
    }

    #[test]
    fn test_matcher_is_ok() {
        use super::matcher;
        use super::Input;
        use std::collections::HashMap;

        let input = Input {
            arg: "generate".to_string(),
            params: HashMap::from([
                ("--password".to_string(), "".to_string()),
                ("--username".to_string(), "test".to_string()),
            ]),
        };
        let command = matcher(&input);
        println!("{:?}", command);

        assert!(command.is_ok());
        assert_eq!(command.unwrap().len(), 16);
    }

    #[test]
    fn test_matcher_is_err_when_input_is_empty() {
        use super::matcher;
        use super::Input;
        use std::collections::HashMap;

        let input = Input {
            arg: "".to_string(),
            params: HashMap::from([
                ("--password".to_string(), "".to_string()),
                ("--username".to_string(), "test".to_string()),
            ]),
        };
        let command = matcher(&input);
        println!("{:?}", command);

        assert!(command.is_err());
    }
}
