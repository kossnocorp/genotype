use crate::*;

mod parse;
mod resolve;

use miette::NamedSource;
pub use parse::GTModuleParse;
pub use resolve::*;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModule {
    /// Module identifier. Used to reference the module in the project.
    pub id: GTModuleId,
    /// Module source code.
    // [TODO] Decide if I want to get rid of it and use source code from the outer context instead.
    // The reason is that the source code contains the path that is rendered in the error messages,
    // and that path to be correctly identified by editors, should be relative to the project
    // working directory that can differ in different contexts i.e. depending on the active VS Code
    // workspace project/folder.
    #[deprecated]
    pub source_code: NamedSource<String>,
    pub doc: Option<GTDoc>,
    pub imports: Vec<GTImport>,
    pub aliases: Vec<GTAlias>,
}
