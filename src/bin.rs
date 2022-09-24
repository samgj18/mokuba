use mcore::mstd::{prepare, read_line_from, write_line_to};
use std::io::{stdin, stdout, Result as IOResult};

fn main() -> IOResult<()> {
    let stdio = stdin();
    let reader = stdio.lock();
    let writer = stdout();

    match read_line_from(reader) {
        Ok(answer) => match prepare(&answer) {
            Ok(command) => {
                // convert Vec<String> to Vec<&str>
                println!("command: {:?}", command);

                Ok(())
            }
            Err(e) => write_line_to(writer, &e),
        },
        Err(err) => write_line_to(writer, err.to_string().as_str()),
    }
}
