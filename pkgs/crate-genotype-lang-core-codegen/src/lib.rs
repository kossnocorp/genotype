use genotype_lang_core_tree::*;
use genotype_parser::*;
use miette::Result;

pub trait GtlCodegen: Default {
    type Import: GtlImport;

    type Definition: GtlDefinition;

    fn new() -> Self {
        Default::default()
    }

    fn register_import(&mut self, import: Self::Import);

    fn register_definition(&mut self, definition: Self::Definition);

    fn inject_descriptor(&mut self, descriptor: GTDescriptor) -> Result<String>;

    fn render_module(&self) -> Result<String>;
}
