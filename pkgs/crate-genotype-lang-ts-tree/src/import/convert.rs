use crate::prelude::internal::*;

impl TSConvert<TSImport> for GTImport {
    fn convert(&self, context: &mut TSConvertContext) -> TSImport {
        let reference = match &self.reference {
            GTImportReference::Glob(_) => {
                TSImportReference::Glob(context.resolve_glob(self).into())
            }

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
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_glob() {
        let mut resolve = TSConvertResolve::new();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        assert_ron_snapshot!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GTImportReference::Glob((0, 0).into())
            }
            .convert(&mut TSConvertContext::new(resolve, Default::default())),
            @r#"
        TSImport(
          path: TSPath("./path/to/module"),
          reference: Glob("module"),
        )
        "#
        );
    }

    #[test]
    fn test_convert_names() {
        assert_ron_snapshot!(
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
            @r#"
        TSImport(
          path: TSPath("./path/to/module"),
          reference: Named([
            Name(TSIdentifier("Name")),
            Alias(TSIdentifier("Name"), TSIdentifier("Alias")),
          ]),
        )
        "#
        );
    }

    #[test]
    fn test_convert_name() {
        assert_ron_snapshot!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GTIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut Default::default()),
            @r#"
        TSImport(
          path: TSPath("./path/to/module"),
          reference: Named([
            Name(TSIdentifier("Name")),
          ]),
        )
        "#
        );
    }
}
