mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSDoc(pub String, pub bool);

impl RSDoc {
    pub fn new<S: Into<String>>(doc: S, module: bool) -> Self {
        RSDoc(doc.into(), module)
    }
}

impl From<&str> for RSDoc {
    fn from(str: &str) -> Self {
        RSDoc(str.into(), false)
    }
}
