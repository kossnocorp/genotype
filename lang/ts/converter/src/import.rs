use genotype_lang_ts_tree::{
    definition::TSDefinition, import::TSImport, TSImportName, TSImportReference,
};
use genotype_parser::tree::{import::GTImport, GTImportReference};

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSImport> for GTImport {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSImport
    where
        HoistFn: Fn(TSDefinition),
    {
        let reference = match &self.reference {
            GTImportReference::Glob => {
                // [TODO]
                TSImportReference::Glob(resolve.globs.get(&self.path).unwrap().clone())
            }

            GTImportReference::Names(names) => TSImportReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(resolve, hoist))
                    .collect::<Vec<_>>(),
            ),

            GTImportReference::Name(name) => {
                TSImportReference::Named(vec![TSImportName::Name(name.convert(resolve, hoist))])
            }
        };

        TSImport {
            path: self.path.convert(resolve, hoist),
            reference,
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
        let mut resolve = TSConvertResolve::new();
        resolve
            .globs
            .insert("./path/to/module".into(), "module".into());
        assert_eq!(
            GTImport {
                path: "./path/to/module".into(),
                reference: GTImportReference::Glob
            }
            .convert(&resolve, &|_| {}),
            TSImport {
                path: "./path/to/module.ts".into(),
                reference: TSImportReference::Glob("module".into())
            }
        );
    }

    #[test]
    fn test_convert_names() {
        assert_eq!(
            GTImport {
                path: "./path/to/module".into(),
                reference: GTImportReference::Names(vec![
                    GTImportName::Name(GTIdentifier::new((0, 0).into(), "Name".into())),
                    GTImportName::Alias(
                        GTIdentifier::new((0, 0).into(), "Name".into()),
                        GTIdentifier::new((0, 0).into(), "Alias".into())
                    )
                ])
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSImport {
                path: "./path/to/module.ts".into(),
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
                reference: GTIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSImport {
                path: "./path/to/module.ts".into(),
                reference: TSImportReference::Named(vec![TSImportName::Name("Name".into())])
            }
        );
    }
}
