mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, PartialOrd, Ord)]
pub struct RSIdentifier(pub String);

impl From<&str> for RSIdentifier {
    fn from(str: &str) -> Self {
        RSIdentifier(str.into())
    }
}