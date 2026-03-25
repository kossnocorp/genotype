use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsDoc(pub String, pub bool);

impl RsDoc {
    pub fn new<S: Into<String>>(doc: S, module: bool) -> Self {
        RsDoc(doc.into(), module)
    }
}

impl From<&str> for RsDoc {
    fn from(str: &str) -> Self {
        RsDoc(str.into(), false)
    }
}
