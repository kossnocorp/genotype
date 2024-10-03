mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSPath(pub String);

impl From<&str> for TSPath {
    fn from(str: &str) -> Self {
        TSPath(str.into())
    }
}
