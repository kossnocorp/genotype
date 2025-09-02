mod convert;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TSPath(pub String);

impl TSPath {
    pub fn is_external(&self) -> bool {
        !self.0.starts_with("./") && !self.0.starts_with("../") && !self.0.starts_with("/")
    }
}

impl From<&str> for TSPath {
    fn from(str: &str) -> Self {
        TSPath(str.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn is_external() {
        assert_eq!(TSPath::from("some/path").is_external(), true);
        assert_eq!(TSPath::from("./some/path").is_external(), false);
        assert_eq!(TSPath::from("../some/path").is_external(), false);
        assert_eq!(TSPath::from("/some/path").is_external(), false);
    }
}
