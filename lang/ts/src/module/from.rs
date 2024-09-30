use genotype_parser::tree::module::GTModule;

use super::TSModule;

impl From<GTModule> for TSModule {
    fn from(_module: GTModule) -> Self {
        TSModule {
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
                doc: None,
                imports: vec![],
                aliases: vec![],
            }),
            TSModule {
                doc: None,
                imports: vec![],
                definitions: vec![],
            }
        );
    }
}
