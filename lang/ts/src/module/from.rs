use genotype_parser::tree::module::Module;

use super::TSModule;

impl From<Module> for TSModule {
    fn from(module: Module) -> Self {
        TSModule {
            path: "./module.ts".to_string(),
            doc: None,
            imports: vec![],
            definitions: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::module::Module;

    #[test]
    fn test_from() {
        assert_eq!(
            TSModule::from(Module {
                path: "./module".to_string(),
                doc: None,
                imports: vec![],
                aliases: vec![],
            }),
            TSModule {
                path: "./module.ts".to_string(),
                doc: None,
                imports: vec![],
                definitions: vec![],
            }
        );
    }
}
