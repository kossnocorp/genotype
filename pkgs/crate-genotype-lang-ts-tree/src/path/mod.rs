use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct TsPath(pub Arc<str>);

impl TsPath {
    pub fn is_external(&self) -> bool {
        !self.0.starts_with("./") && !self.0.starts_with("../") && !self.0.starts_with("/")
    }
}

impl From<&str> for TsPath {
    fn from(str: &str) -> Self {
        TsPath(str.into())
    }
}

impl From<String> for TsPath {
    fn from(str: String) -> Self {
        TsPath(str.into())
    }
}

impl From<Arc<str>> for TsPath {
    fn from(str: Arc<str>) -> Self {
        TsPath(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn is_external() {
        assert_eq!(TsPath::from("some/path").is_external(), true);
        assert_eq!(TsPath::from("./some/path").is_external(), false);
        assert_eq!(TsPath::from("../some/path").is_external(), false);
        assert_eq!(TsPath::from("/some/path").is_external(), false);
    }
}
