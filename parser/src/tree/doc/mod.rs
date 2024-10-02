mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTDoc(pub String);

impl From<&str> for GTDoc {
    fn from(str: &str) -> Self {
        GTDoc(str.into())
    }
}
