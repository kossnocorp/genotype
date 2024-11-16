mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSFieldName(pub String);

impl From<&str> for RSFieldName {
    fn from(str: &str) -> Self {
        RSFieldName(str.into())
    }
}
