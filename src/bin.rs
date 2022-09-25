use mcore::mstd::{deserialize, matcher, read_line_from, write_line_to};
use std::io::{stdin, stdout, Result as IOResult};

fn main() -> IOResult<()> {
    let stdio = stdin();
    let reader = stdio.lock();
    let writer = stdout();

    match read_line_from(reader) {
        Ok(answer) => match deserialize(&answer) {
            Ok(command) => {
                // TODO: Add help command to show all commands and their descriptions and help command per command
                match matcher(&command) {
                    Ok(input) => write_line_to(writer, &input),
                    Err(error) => write_line_to(writer, &error),
                }
            }
            Err(e) => write_line_to(writer, &e),
        },
        Err(err) => write_line_to(writer, err.to_string().as_str()),
    }
}
