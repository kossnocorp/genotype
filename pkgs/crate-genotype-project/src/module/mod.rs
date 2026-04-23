use crate::prelude::internal::*;

mod definition;
pub use definition::*;

mod identifier;
pub use identifier::*;

mod parse;
pub use parse::*;

mod resolve;
pub use resolve::*;

mod state;
pub use state::*;

mod error;
pub use error::*;

/// Project module.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpModule {
    /// Module path relative to the src directory.
    /// TODO: Make it relative to the project root (config) path.
    pub path: GtpModulePath,
    /// Module tree node.
    pub module: GtModule,
    /// Project module resolve.
    pub resolve: GtpModuleResolve,
    /// Module source code.
    /// TODO: After implementing workspace, find a better place for it.
    #[serde(serialize_with = "genotype_parser::miette_serde::serialize_named_source")]
    // TODO: Use #[deprecated] and remove usage
    pub source_code: NamedSource<String>,
}

impl GtpModule {
    pub fn try_new(
        project_resolve: &GtpResolve,
        modules: &[GtpModuleParse],
        parse: GtpModuleParse,
    ) -> Result<Self> {
        let mut module_resolve = GtpModuleResolve::try_new(modules, &parse)
            .map_err(|err| err.with_source_code(parse.1.source_code.clone()))?;

        // Combine these two ^v

        let mut visitor = GtpResolveVisitor::new(parse.1.module.id.clone(), project_resolve);
        let parse = parse;
        parse.1.module.traverse(&mut visitor);

        if let Some(error) = visitor.error() {
            return Err(
                miette::Report::new(error.clone()).with_source_code(parse.1.source_code.clone())
            );
        }

        module_resolve.definitions = visitor.drain_definitions();
        module_resolve.reference_definition_ids = visitor.get_reference_definition_ids();

        Ok(GtpModule {
            path: parse.0,
            module: parse.1.module,
            resolve: module_resolve,
            source_code: parse.1.source_code.clone(),
        })
    }
}
