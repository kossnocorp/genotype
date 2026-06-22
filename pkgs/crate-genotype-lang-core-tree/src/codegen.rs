// use crate::*;
// use genotype_parser::*;
// use miette::Result;

// pub trait GtlCodegen: Default {
//     type Definition: GtlDefinition;
//     type Import: GtlImport;
//     type Error: GtlError;

//     fn new() -> Self {
//         Default::default()
//     }

//     fn register_import(&mut self, import: Self::Import);

//     fn register_definition(&mut self, definition: Self::Definition);

//     fn inject_descriptor(&mut self, descriptor: GtDescriptor) -> Result<String, Self::Error>;

//     fn render_module(&self) -> Result<String, Self::Error>;
// }
