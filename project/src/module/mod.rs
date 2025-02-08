use genotype_parser::GTModule;
use genotype_visitor::traverse::GTTraverse;
use miette::{NamedSource, Result};

mod parse;
mod path;
mod resolve;

pub use parse::*;
pub use path::*;
pub use resolve::*;

use crate::{visitor::GTProjectResolveVisitor, GTProjectResolve};

#[derive(Debug, PartialEq, Clone)]
pub struct GTProjectModule {
    pub path: GTProjectModulePath,
    pub module: GTModule,
    pub resolve: GTProjectModuleResolve,
    /// Module source code.
    /// [TODO] After implementing workspace, find a better place for it.
    #[deprecated]
    pub source_code: NamedSource<String>,
}

impl GTProjectModule {
    pub fn try_new(
        definitions: &GTProjectResolve,
        modules: &Vec<GTProjectModuleParse>,
        parse: GTProjectModuleParse,
    ) -> Result<Self> {
        let mut resolve = GTProjectModuleResolve::try_new(modules, &parse)
            .map_err(|err| err.with_source_code(parse.1.source_code.clone()))?;

        // Combine these two ^v

        let mut visitor = GTProjectResolveVisitor::new(parse.1.module.id.clone(), &definitions);
        let mut parse = parse;
        parse.1.module.traverse(&mut visitor);

        resolve.references = visitor.drain_references();

        Ok(GTProjectModule {
            path: parse.0,
            module: parse.1.module,
            resolve,
            source_code: parse.1.source_code.clone(),
        })
    }
}
