use crate::prelude::internal::*;

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
    dependencies_config: IndexMap<String, String>,
    naming: TsConfigNaming,
}

impl TsConvertContext {
    pub fn new(resolve: TsConvertResolve, config: &TsConfig) -> Self {
        Self {
            resolve,
            mode: config.lang.mode,
            imports: vec![],
            hoisted: vec![],
            doc: None,
            dependencies_config: config.common.dependencies.clone(),
            naming: config.lang.naming.clone(),
        }
    }

    pub fn mode(&self) -> TsMode {
        self.mode
    }
}

impl GtlConvertContext for TsConvertContext {
    type Import = TsImport;

    fn imports(&self) -> &Vec<TsImport> {
        &self.imports
    }

    fn imports_mut(&mut self) -> &mut Vec<TsImport> {
        &mut self.imports
    }
}

impl Default for TsConvertContext {
    fn default() -> Self {
        Self::new(Default::default(), &Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode() {
        for mode in [TsMode::Types, TsMode::Zod, TsMode::Effect] {
            let mut config = TsConfig::default();
            config.lang.mode = mode;
            let context = TsConvertContext::new(Default::default(), &config);
            assert_eq!(context.mode(), mode);
        }
    }
}
