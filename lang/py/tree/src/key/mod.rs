mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYKey {
    pub name: String,
    pub aliased: Option<String>,
}

impl PYKey {
    pub fn new(name: String, aliased: Option<String>) -> Self {
        PYKey { name, aliased }
    }
}

impl From<&str> for PYKey {
    fn from(str: &str) -> Self {
        PYKey {
            name: str.into(),
            aliased: None,
        }
    }
}
