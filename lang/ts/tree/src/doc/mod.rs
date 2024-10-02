mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSDoc(pub String);

impl From<&str> for TSDoc {
    fn from(str: &str) -> Self {
        TSDoc(str.into())
    }
}
