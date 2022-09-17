#[derive(Debug)]
pub struct GenError {
    pub reason: String,
}

impl GenError {
    pub fn new(reason: &str) -> GenError {
        GenError {
            reason: reason.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Params {
    pub length: u32,
}

impl Params {
    pub fn new(length: u32) -> Params {
        Params { length }
    }
}
