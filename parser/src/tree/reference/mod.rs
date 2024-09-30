use super::name::GTName;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTReference {
    pub path: String,
    pub name: GTName,
}
