use genotype_parser::GTModuleId;

mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct RSPath(pub GTModuleId, pub String);

impl RSPath {
    pub fn join(str: &str, other: &str) -> String {
        format!("{}::{}", str, other)
    }
}
