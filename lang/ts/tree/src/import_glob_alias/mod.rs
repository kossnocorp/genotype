mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportGlobAlias {
    Unresolved,
    Resolved(String),
}
