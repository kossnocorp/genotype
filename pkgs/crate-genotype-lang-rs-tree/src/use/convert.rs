use crate::prelude::internal::*;

impl RsConvert<RsUse> for GtImport {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsUse> {
        let reference = match &self.reference {
            GtImportReference::Glob(_) => RsUseReference::Module,

            GtImportReference::Names(_, names) => RsUseReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(context))
                    .collect::<Result<Vec<_>>>()?,
            ),

            GtImportReference::Name(_, name) => {
                RsUseReference::Named(vec![RsUseName::Name(name.convert(context)?)])
            }
        };

        let path = self.path.convert(context)?;

        Ok(RsUse {
            reference,
            dependency: RsDependencyIdent::Local(path),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_glob() {
        let mut resolve = RsConvertResolve::default();
        resolve.globs.insert(
            GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::new(
                    (0, 0).into(),
                    GtPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
                reference: GtImportReference::Glob((0, 0).into())
            }
            .convert(&mut context)
            .unwrap(),
            @r#"
        RsUse(
          dependency: Local(RsPath(GtModuleId("module/path"), "super::path::to::module")),
          reference: Module,
        )
        "#
        );
    }

    #[test]
    fn test_convert_names() {
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::new(
                    (0, 0).into(),
                    GtPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
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
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        RsUse(
          dependency: Local(RsPath(GtModuleId("module/path"), "super::path::to::module")),
          reference: Named([
            Name(RsIdentifier("Name")),
            Alias(RsIdentifier("Name"), RsIdentifier("Alias")),
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
                path: GtPath::new(
                    (0, 0).into(),
                    GtPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
                reference: GtIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @r#"
        RsUse(
          dependency: Local(RsPath(GtModuleId("module/path"), "super::path::to::module")),
          reference: Named([
            Name(RsIdentifier("Name")),
          ]),
        )
        "#
        );
    }
}
