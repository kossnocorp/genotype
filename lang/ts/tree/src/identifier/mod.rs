mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSIdentifier(pub String);

impl From<&str> for TSIdentifier {
    fn from(str: &str) -> Self {
        TSIdentifier(str.into())
    }
}
