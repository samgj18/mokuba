extern crate mokuba;

use std::{io, io::Result as IOResult};

fn main() -> IOResult<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}
