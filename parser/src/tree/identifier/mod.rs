mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTIdentifier(pub String);

impl From<&str> for GTIdentifier {
    fn from(str: &str) -> Self {
        GTIdentifier(str.into())
    }
}
