use super::property::GTProperty;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTObject {
    pub properties: Vec<GTProperty>,
}
