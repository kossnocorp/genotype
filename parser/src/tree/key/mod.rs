mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTKey(pub String);

impl From<&str> for GTKey {
    fn from(str: &str) -> Self {
        GTKey(str.into())
    }
}
