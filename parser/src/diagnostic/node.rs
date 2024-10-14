#[derive(Debug, PartialEq)]
pub enum GTNode {
    Extension,
    Literal,
    Path,
    Primitive,
}

impl GTNode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Extension => "extension",
            Self::Literal => "literal",
            Self::Path => "path",
            Self::Primitive => "primitive",
        }
    }
}
