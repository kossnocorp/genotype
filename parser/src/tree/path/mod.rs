mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTPath(pub String);

impl From<&str> for GTPath {
    fn from(str: &str) -> Self {
        GTPath(str.into())
    }
}
