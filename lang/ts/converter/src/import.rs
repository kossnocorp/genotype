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
            GTImportReference::Glob(_) => {
                // [TODO]
                TSImportReference::Glob(resolve.globs.get(&self.path).unwrap().clone())
            }

            GTImportReference::Names(_, names) => TSImportReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(resolve, hoist))
                    .collect::<Vec<_>>(),
            ),

            GTImportReference::Name(_, name) => {
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
    use genotype_parser::*;

    #[test]
    fn test_convert_glob() {
        let mut resolve = TSConvertResolve::new();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        assert_eq!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GTImportReference::Glob((0, 0).into())
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
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GTImportReference::Names(
                    (0, 0).into(),
                    vec![
                        GTImportName::Name(
                            (0, 0).into(),
                            GTIdentifier::new((0, 0).into(), "Name".into())
                        ),
                        GTImportName::Alias(
                            (0, 0).into(),
                            GTIdentifier::new((0, 0).into(), "Name".into()),
                            GTIdentifier::new((0, 0).into(), "Alias".into())
                        )
                    ]
                )
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
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
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
