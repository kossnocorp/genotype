#[derive(Debug, PartialEq)]
pub enum GTNode {
    Alias,
    Array,
    Extension,
    Import,
    Literal,
    Object,
    Path,
    Primitive,
    Property,
}

impl GTNode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Alias => "alias",
            Self::Array => "array",
            Self::Extension => "extension",
            Self::Import => "import",
            Self::Literal => "literal",
            Self::Object => "object",
            Self::Path => "path",
            Self::Primitive => "primitive",
            Self::Property => "property",
        }
    }
}
