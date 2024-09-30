use genotype_lang_ts_tree::module::TSModule;
use genotype_parser::tree::module::GTModule;

#[derive(Debug, PartialEq, Clone)]
pub struct TSModuleParse {
    pub module: TSModule,
}

impl From<GTModule> for TSModuleParse {
    fn from(_module: GTModule) -> Self {
        TSModuleParse {
            module: TSModule {
                doc: None,
                imports: vec![],
                definitions: vec![],
            },
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
            TSModuleParse::from(GTModule {
                doc: None,
                imports: vec![],
                aliases: vec![],
            }),
            TSModuleParse {
                module: TSModule {
                    doc: None,
                    imports: vec![],
                    definitions: vec![],
                }
            }
        );
    }
}
