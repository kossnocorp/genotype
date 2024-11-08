mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSKey(pub String);

impl From<&str> for RSKey {
    fn from(str: &str) -> Self {
        RSKey(str.into())
    }
}
