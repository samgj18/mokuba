use mmacro::ConstructorM;

#[derive(Debug, ConstructorM)]
pub struct GenerateParams {
    pub length: u32,
    pub username: Option<String>,
}

impl Default for GenerateParams {
    fn default() -> Self {
        GenerateParams {
            length: 10,
            username: None,
        }
    }
}
