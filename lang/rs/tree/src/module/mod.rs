use genotype_parser::GTModuleId;

use crate::{definition::RSDefinition, r#use::RSUse, RSDoc};

pub mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSModule {
    pub id: GTModuleId,
    pub doc: Option<RSDoc>,
    pub imports: Vec<RSUse>,
    pub definitions: Vec<RSDefinition>,
}
