mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSDoc(pub String);

impl From<&str> for RSDoc {
    fn from(str: &str) -> Self {
        RSDoc(str.into())
    }
}
