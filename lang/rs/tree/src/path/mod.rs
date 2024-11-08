mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct RSPath(pub String);

impl From<&str> for RSPath {
    fn from(str: &str) -> Self {
        RSPath(str.into())
    }
}
