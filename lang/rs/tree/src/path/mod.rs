mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct RSPath(pub String);

impl RSPath {
    pub fn join(&self, other: &RSPath) -> RSPath {
        format!("{}::{}", self.0, other.0).into()
    }
}

impl From<&str> for RSPath {
    fn from(str: &str) -> Self {
        RSPath(str.into())
    }
}

impl From<String> for RSPath {
    fn from(str: String) -> Self {
        RSPath(str)
    }
}
