mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYKey(pub String);

impl From<&str> for PYKey {
    fn from(str: &str) -> Self {
        PYKey(str.into())
    }
}
