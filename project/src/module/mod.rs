mod parse;
mod path;
mod resolve;

use genotype_parser::tree::GTModule;
pub use parse::*;
pub use path::*;
pub use resolve::*;

#[derive(Debug, PartialEq, Clone)]
pub struct GTProjectModule {
    pub path: GTProjectModulePath,
    pub module: GTModule,
    pub resolve: GTProjectModuleResolve,
}

impl GTProjectModule {
    pub fn try_new(
        modules: &Vec<GTProjectModuleParse>,
        parse: GTProjectModuleParse,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let resolve = GTProjectModuleResolve::try_new(modules, &parse)?;
        Ok(GTProjectModule {
            path: parse.0,
            module: parse.1.module,
            resolve,
        })
    }
}
