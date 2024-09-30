use genotype_lang_ts_tree::{definition::TSDefinition, import::TSImport};
use genotype_parser::tree::import::GTImport;

use crate::convert::TSConvert;

impl TSConvert<TSImport> for GTImport {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSImport
    where
        HoistFn: Fn(TSDefinition),
    {
        TSImport {
            path: self.path.clone(),
            reference: self.reference.convert(hoist),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::{
        import_glob_alias::TSImportGlobAlias, import_name::TSImportName,
        import_reference::TSImportReference, name::TSName,
    };
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::{
        import_name::GTImportName, import_reference::GTImportReference, name::GTName,
    };

    #[test]
    fn test_convert_glob() {
        assert_eq!(
            GTImport {
                path: "./path/to/module".to_string(),
                reference: GTImportReference::Glob
            }
            .convert(&|_| {}),
            TSImport {
                path: "./path/to/module".to_string(),
                reference: TSImportReference::Glob(TSImportGlobAlias::Unresolved)
            }
        );
    }

    #[test]
    fn test_convert_names() {
        assert_eq!(
            GTImport {
                path: "./path/to/module".to_string(),
                reference: GTImportReference::Names(vec![
                    GTImportName::Name(GTName("Name".to_string())),
                    GTImportName::Alias(GTName("Name".to_string()), GTName("Alias".to_string()))
                ])
            }
            .convert(&|_| {}),
            TSImport {
                path: "./path/to/module".to_string(),
                reference: TSImportReference::Named(vec![
                    TSImportName::Name(TSName("Name".to_string())),
                    TSImportName::Alias(TSName("Name".to_string()), TSName("Alias".to_string()))
                ])
            }
        );
    }

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTImport {
                path: "./path/to/module".to_string(),
                reference: GTImportReference::Name(GTName("Name".to_string()))
            }
            .convert(&|_| {}),
            TSImport {
                path: "./path/to/module".to_string(),
                reference: TSImportReference::Named(vec![TSImportName::Name(TSName(
                    "Name".to_string()
                ))])
            }
        );
    }
}
