mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYDoc(pub String);

impl From<&str> for PYDoc {
    fn from(str: &str) -> Self {
        PYDoc(str.into())
    }
}
