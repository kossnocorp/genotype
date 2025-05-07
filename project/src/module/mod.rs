use genotype_parser::GTModule;
use genotype_path::{GtModulePath, GtSrcRelativePath};
use genotype_visitor::traverse::GTTraverse;
use miette::{NamedSource, Result};

mod definition;
pub use definition::*;

mod identifier;
pub use identifier::*;

mod parse;
pub use parse::*;

mod resolve;
pub use resolve::*;

use crate::{visitor::GTPResolveVisitor, GTPResolve};

#[derive(Debug, PartialEq, Clone)]
pub struct GtProjectModule {
    pub path: GtModulePath,
    pub module: GTModule,
    /// Project module resolve.
    pub resolve: GTPModuleResolve,
    /// Module source code.
    /// [TODO] After implementing workspace, find a better place for it.
    #[deprecated]
    pub source_code: NamedSource<String>,
}

impl GtProjectModule {
    pub fn try_new(
        project_resolve: &GTPResolve,
        modules: &Vec<GTProjectModuleParse>,
        parse: GTProjectModuleParse,
    ) -> Result<Self> {
        let mut module_resolve = GTPModuleResolve::try_new(modules, &parse)
            .map_err(|err| err.with_source_code(parse.1.source_code.clone()))?;

        // Combine these two ^v

        let mut visitor = GTPResolveVisitor::new(parse.1.module.id.clone(), &project_resolve);
        let mut parse = parse;
        parse.1.module.traverse(&mut visitor);

        module_resolve.definitions = visitor.drain_definitions();

        Ok(GtProjectModule {
            path: parse.0,
            module: parse.1.module,
            resolve: module_resolve,
            source_code: parse.1.source_code.clone(),
        })
    }
}
