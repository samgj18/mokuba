use command::Command;
use mcore::executable::Execute;
use mcore::mstd::{read_line_from, write_line_to};
use std::io::{stdin, stdout, Error, ErrorKind, Result as IOResult};

pub mod command;

fn main() -> IOResult<()> {
    let stdio = stdin();
    let reader = stdio.lock();
    let writer = stdout();

    match read_line_from(reader) {
        Ok(answer) => match Command::parse(&answer) {
            Ok(command) => match command.execute() {
                Ok(result) => write_line_to(writer, &result),
                Err(e) => Err(Error::new(ErrorKind::Other, e)),
            },
            Err(e) => write_line_to(writer, &e),
        },
        Err(err) => write_line_to(writer, err.to_string().as_str()),
    }
}
