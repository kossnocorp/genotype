mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYPath(pub String);

impl From<&str> for PYPath {
    fn from(str: &str) -> Self {
        PYPath(str.into())
    }
}
