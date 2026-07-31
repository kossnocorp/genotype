use crate::prelude::internal::*;

// TODO: Consider using Arc<str> for GtModuleId for better memory efficiency. The problem is that
// Genotype has no way to tell to use Arc.

// TODO: Add option to add derives to a specific type with Genotype

impl GtModuleId {
    pub fn as_str_without_ext(&self) -> String {
        self.0.trim_end_matches(".type").to_owned()
    }
}

impl Display for GtModuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for GtModuleId {
    fn from(s: &str) -> Self {
        GtModuleId(s.into())
    }
}

impl From<String> for GtModuleId {
    fn from(s: String) -> Self {
        GtModuleId(s)
    }
}

#[allow(clippy::derivable_impls)]
impl Default for GtModuleId {
    fn default() -> Self {
        GtModuleId(String::new())
    }
}

impl AsRef<str> for GtModuleId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for GtModuleId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl Eq for GtModuleId {}

impl Hash for GtModuleId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
