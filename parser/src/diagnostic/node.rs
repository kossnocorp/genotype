#[derive(Debug, PartialEq)]
pub enum GTNode {
    Extension,
    Primitive,
}

impl GTNode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Extension => "extension",
            Self::Primitive => "primitive",
        }
    }
}
