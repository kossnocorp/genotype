use crate::{name::TSName, type_descriptor::TSTypeDescriptor};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProperty {
    pub name: TSName,
    pub descriptor: TSTypeDescriptor,
    pub optional: bool,
}
