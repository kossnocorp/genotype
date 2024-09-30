use genotype_parser::tree::module::GTModule;

use super::TSModule;

impl From<GTModule> for TSModule {
    fn from(module: GTModule) -> Self {
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
    use genotype_parser::tree::module::GTModule;

    #[test]
    fn test_from() {
        assert_eq!(
            TSModule::from(GTModule {
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
