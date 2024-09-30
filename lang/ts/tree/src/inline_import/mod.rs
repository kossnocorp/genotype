use crate::name::TSName;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSInlineImport {
    pub path: String,
    pub name: TSName,
}
