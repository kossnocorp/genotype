use crate::name::TSName;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportName {
    Name(TSName),
    Alias(TSName, TSName),
}
