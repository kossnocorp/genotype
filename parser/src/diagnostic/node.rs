#[derive(Debug, PartialEq)]
pub enum GTNode {
    Alias,
    Array,
    Descriptor,
    Extension,
    Import,
    Literal,
    Module,
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
            Self::Descriptor => "descriptor",
            Self::Extension => "extension",
            Self::Import => "import",
            Self::Literal => "literal",
            Self::Module => "module",
            Self::Object => "object",
            Self::Path => "path",
            Self::Primitive => "primitive",
            Self::Property => "property",
        }
    }
}
