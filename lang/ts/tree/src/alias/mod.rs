use crate::{name::TSName, type_descriptor::TSTypeDescriptor};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSAlias {
    pub name: TSName,
    pub descriptor: TSTypeDescriptor,
}
