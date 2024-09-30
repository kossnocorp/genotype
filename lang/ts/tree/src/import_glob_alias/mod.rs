use crate::name::TSName;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportGlobAlias {
    Resolved(TSName),
    Unresolved,
}
