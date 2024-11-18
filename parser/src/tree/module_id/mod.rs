#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTModuleId(pub String);

impl From<&str> for GTModuleId {
    fn from(s: &str) -> Self {
        GTModuleId(s.to_string())
    }
}
