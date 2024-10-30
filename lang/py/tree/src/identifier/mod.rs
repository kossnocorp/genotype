mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, PartialOrd, Ord)]
pub struct PYIdentifier(pub String);

impl From<&str> for PYIdentifier {
    fn from(str: &str) -> Self {
        PYIdentifier(str.into())
    }
}
