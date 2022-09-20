use std::io::{BufRead, Write};

use crate::model::error::GetInputError;

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
            super::model::error::ErrorCode::UnableToReadInput,
            Some(&format!("Unable to read input properly with error: {}", e)),
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
pub fn write_line_to<W: Write>(mut writer: W, line: &str) -> Result<(), GetInputError> {
    writer.write_all(line.as_bytes()).map_err(|e| {
        GetInputError::new(
            super::model::error::ErrorCode::UnableToWriteOutput,
            Some(&format!("Unable to write input properly with error: {}", e)),
        )
    })?;
    Ok(())
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
}
