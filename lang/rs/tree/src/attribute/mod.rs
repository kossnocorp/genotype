mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSAttribute(pub String);

impl From<&str> for RSAttribute {
    fn from(value: &str) -> Self {
        RSAttribute(value.into())
    }
}

impl From<String> for RSAttribute {
    fn from(value: String) -> Self {
        RSAttribute(value)
    }
}
