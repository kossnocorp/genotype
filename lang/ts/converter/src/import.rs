use genotype_lang_ts_tree::{definition::TSDefinition, import::TSImport};
use genotype_parser::tree::import::GTImport;

use crate::convert::TSConvert;

impl TSConvert<TSImport> for GTImport {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSImport
    where
        HoistFn: Fn(TSDefinition),
    {
        TSImport {
            path: self.path.convert(hoist),
            reference: self.reference.convert(hoist),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::*;

    #[test]
    fn test_convert_glob() {
        assert_eq!(
            GTImport {
                path: "./path/to/module".into(),
                reference: GTImportReference::Glob
            }
            .convert(&|_| {}),
            TSImport {
                path: "./path/to/module".into(),
                reference: TSImportReference::Glob(TSImportGlobAlias::Unresolved)
            }
        );
    }

    #[test]
    fn test_convert_names() {
        assert_eq!(
            GTImport {
                path: "./path/to/module".into(),
                reference: GTImportReference::Names(vec![
                    GTImportName::Name("Name".into()),
                    GTImportName::Alias("Name".into(), "Alias".into())
                ])
            }
            .convert(&|_| {}),
            TSImport {
                path: "./path/to/module".into(),
                reference: TSImportReference::Named(vec![
                    TSImportName::Name("Name".into()),
                    TSImportName::Alias("Name".into(), "Alias".into())
                ])
            }
        );
    }

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTImport {
                path: "./path/to/module".into(),
                reference: GTImportReference::Name("Name".into())
            }
            .convert(&|_| {}),
            TSImport {
                path: "./path/to/module".into(),
                reference: TSImportReference::Named(vec![TSImportName::Name("Name".into())])
            }
        );
    }
}
