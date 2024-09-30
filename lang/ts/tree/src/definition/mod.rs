use crate::definition_descriptor::TSDefinitionDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSDefinition {
    pub doc: Option<String>,
    pub descriptor: TSDefinitionDescriptor,
}
