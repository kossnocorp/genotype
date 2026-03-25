use crate::prelude::internal::*;

impl TsConvert<TsImport> for GtImport {
    fn convert(&self, context: &mut TsConvertContext) -> TsImport {
        let reference = match &self.reference {
            GtImportReference::Glob(_) => {
                TsImportReference::Glob(context.resolve_glob(self).into())
            }

            GtImportReference::Names(_, names) => TsImportReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(context))
                    .collect::<Vec<_>>(),
            ),

            GtImportReference::Name(_, name) => {
                TsImportReference::Named(vec![TsImportName::Name(name.convert(context))])
            }
        };

        TsImport {
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
        let mut resolve = TsConvertResolve::new();
        resolve.globs.insert(
            GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GtImportReference::Glob((0, 0).into())
            }
            .convert(&mut TsConvertContext::new(resolve, Default::default())),
            @r#"
        TsImport(
          path: TsPath("./path/to/module"),
          reference: Glob("module"),
        )
        "#
        );
    }

    #[test]
    fn test_convert_names() {
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GtImportReference::Names(
                    (0, 0).into(),
                    vec![
                        GtImportName::Name(
                            (0, 0).into(),
                            GtIdentifier::new((0, 0).into(), "Name".into())
                        ),
                        GtImportName::Alias(
                            (0, 0).into(),
                            GtIdentifier::new((0, 0).into(), "Name".into()),
                            GtIdentifier::new((0, 0).into(), "Alias".into())
                        )
                    ]
                )
            }
            .convert(&mut Default::default()),
            @r#"
        TsImport(
          path: TsPath("./path/to/module"),
          reference: Named([
            Name(TsIdentifier("Name")),
            Alias(TsIdentifier("Name"), TsIdentifier("Alias")),
          ]),
        )
        "#
        );
    }

    #[test]
    fn test_convert_name() {
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GtIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut Default::default()),
            @r#"
        TsImport(
          path: TsPath("./path/to/module"),
          reference: Named([
            Name(TsIdentifier("Name")),
          ]),
        )
        "#
        );
    }
}
