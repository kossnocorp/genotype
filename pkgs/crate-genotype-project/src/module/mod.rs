use genotype_parser::GtModule;
use genotype_parser::visitor::Traverse;
use genotype_path::{GtModulePath, GtSrcRelativePath};
use miette::{NamedSource, Result};

mod definition;
pub use definition::*;

mod identifier;
pub use identifier::*;

mod parse;
pub use parse::*;

mod resolve;
pub use resolve::*;
use serde::Serialize;

use crate::{GtpResolve, visitor::GtpResolveVisitor};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtProjectModule {
    pub path: GtModulePath,
    pub module: GtModule,
    /// Project module resolve.
    pub resolve: GtpModuleResolve,
    /// Module source code.
    /// [TODO] After implementing workspace, find a better place for it.
    #[serde(serialize_with = "genotype_parser::miette_serde::serialize_named_source")]
    #[deprecated]
    pub source_code: NamedSource<String>,
}

impl GtProjectModule {
    pub fn try_new(
        project_resolve: &GtpResolve,
        modules: &Vec<GtProjectModuleParse>,
        parse: GtProjectModuleParse,
    ) -> Result<Self> {
        let mut module_resolve = GtpModuleResolve::try_new(modules, &parse)
            .map_err(|err| err.with_source_code(parse.1.source_code.clone()))?;

        // Combine these two ^v

        let mut visitor = GtpResolveVisitor::new(parse.1.module.id.clone(), &project_resolve);
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
