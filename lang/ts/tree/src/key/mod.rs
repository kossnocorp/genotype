mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSKey(pub String);

impl From<&str> for TSKey {
    fn from(str: &str) -> Self {
        TSKey(str.into())
    }
}
