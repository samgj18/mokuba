use mmacro::ConstructorM;

#[derive(Debug, ConstructorM)]
pub struct PassParams {
    pub length: u32,
}

impl Default for PassParams {
    fn default() -> Self {
        PassParams { length: 10 }
    }
}
