use crate::prelude::internal::*;
use std::collections::HashMap;

mod doc;
pub mod hoisting;
mod resolve;

#[derive(Debug, Clone, PartialEq)]
pub struct TsConvertContext {
    resolve: TsConvertResolve,
    mode: TsMode,
    imports: Vec<TsImport>,
    hoisted: Vec<TsDefinition>,
    doc: Option<TsDoc>,
    dependencies_config: HashMap<String, String>,
}

impl TsConvertContext {
    pub fn new(resolve: TsConvertResolve, config: &TsConfig) -> Self {
        Self {
            resolve,
            mode: config.lang.mode.clone(),
            imports: vec![],
            hoisted: vec![],
            doc: None,
            dependencies_config: config.common.dependencies.clone(),
        }
    }

    pub fn is_zod_mode(&self) -> bool {
        self.mode == TsMode::Zod
    }
}

impl GtlConvertContext for TsConvertContext {
    type Import = TsImport;

    fn imports(&self) -> &Vec<Self::Import> {
        &self.imports
    }

    fn imports_mut(&mut self) -> &mut Vec<Self::Import> {
        &mut self.imports
    }
}

impl Default for TsConvertContext {
    fn default() -> Self {
        Self::new(Default::default(), &Default::default())
    }
}
