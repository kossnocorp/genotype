use genotype_parser::GTModule;
use genotype_visitor::traverse::GTTraverse;
use miette::{NamedSource, Result};

mod definition;
mod identifier;
mod parse;
mod path;
mod resolve;

pub use definition::*;
pub use identifier::*;
pub use parse::*;
pub use path::*;
pub use resolve::*;

use crate::{visitor::GTPResolveVisitor, GTPResolve};

#[derive(Debug, PartialEq, Clone)]
pub struct GTProjectModule {
    pub path: GTPModulePath,
    pub module: GTModule,
    /// Project module resolve.
    pub resolve: GTPModuleResolve,
    /// Module source code.
    /// [TODO] After implementing workspace, find a better place for it.
    #[deprecated]
    pub source_code: NamedSource<String>,
}

impl GTProjectModule {
    pub fn try_new(
        definitions: &GTPResolve,
        modules: &Vec<GTProjectModuleParse>,
        parse: GTProjectModuleParse,
    ) -> Result<Self> {
        let mut resolve = GTPModuleResolve::try_new(modules, &parse)
            .map_err(|err| err.with_source_code(parse.1.source_code.clone()))?;

        // Combine these two ^v

        let mut visitor = GTPResolveVisitor::new(parse.1.module.id.clone(), &definitions);
        let mut parse = parse;
        parse.1.module.traverse(&mut visitor);

        resolve.definitions = visitor.drain_definitions();

        Ok(GTProjectModule {
            path: parse.0,
            module: parse.1.module,
            resolve,
            source_code: parse.1.source_code.clone(),
        })
    }
}
