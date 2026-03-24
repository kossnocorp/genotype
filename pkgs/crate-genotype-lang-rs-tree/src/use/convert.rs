use crate::prelude::internal::*;

impl RSConvert<RSUse> for GTImport {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSUse> {
        let reference = match &self.reference {
            GTImportReference::Glob(_) => RSUseReference::Module,

            GTImportReference::Names(_, names) => RSUseReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(context))
                    .collect::<Result<Vec<_>>>()?,
            ),

            GTImportReference::Name(_, name) => {
                RSUseReference::Named(vec![RSUseName::Name(name.convert(context)?)])
            }
        };

        let path = self.path.convert(context)?;

        Ok(RSUse {
            reference,
            dependency: RSDependencyIdent::Local(path),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_glob() {
        let mut resolve = RSConvertResolve::default();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::new(
                    (0, 0).into(),
                    GTPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
                reference: GTImportReference::Glob((0, 0).into())
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RSUse(
          dependency: Local(RSPath(GTModuleId("module/path"), "super::path::to::module")),
          reference: Module,
        )
        "#
        );
    }

    #[test]
    fn test_convert_names() {
        assert_ron_snapshot!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::new(
                    (0, 0).into(),
                    GTPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
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
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        RSUse(
          dependency: Local(RSPath(GTModuleId("module/path"), "super::path::to::module")),
          reference: Named([
            Name(RSIdentifier("Name")),
            Alias(RSIdentifier("Name"), RSIdentifier("Alias")),
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
                path: GTPath::new(
                    (0, 0).into(),
                    GTPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
                reference: GTIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        RSUse(
          dependency: Local(RSPath(GTModuleId("module/path"), "super::path::to::module")),
          reference: Named([
            Name(RSIdentifier("Name")),
          ]),
        )
        "#
        );
    }
}
