use crate::prelude::internal::*;

impl TSConvert<TSImport> for GTImport {
    fn convert(&self, context: &mut TSConvertContext) -> TSImport {
        let reference = match &self.reference {
            GTImportReference::Glob(_) => TSImportReference::Glob(context.resolve_glob(self)),

            GTImportReference::Names(_, names) => TSImportReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(context))
                    .collect::<Vec<_>>(),
            ),

            GTImportReference::Name(_, name) => {
                TSImportReference::Named(vec![TSImportName::Name(name.convert(context))])
            }
        };

        TSImport {
            path: self.path.convert(context),
            reference,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
            .convert(&mut TSConvertContext::new(resolve, Default::default())),
            TSImport {
                path: "./path/to/module".into(),
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
            .convert(&mut Default::default()),
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
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GTIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut Default::default()),
            TSImport {
                path: "./path/to/module".into(),
                reference: TSImportReference::Named(vec![TSImportName::Name("Name".into())])
            }
        );
    }
}
