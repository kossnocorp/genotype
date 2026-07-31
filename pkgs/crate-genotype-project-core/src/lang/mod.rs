use crate::prelude::internal::*;

mod config;
pub use config::*;

// TODO: Automatically generate some of these impls with litty
// TODO: Add option to add derives to a specific type with Genotype

impl PartialEq<str> for GtpLang {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl Eq for GtpLang {}

impl PartialOrd for GtpLang {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GtpLang {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for GtpLang {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl Copy for GtpLang {}
