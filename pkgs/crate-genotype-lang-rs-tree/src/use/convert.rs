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

    #[test]
    fn test_convert_glob() {
        let mut resolve = RsConvertResolve::default();
        resolve.globs.insert(
            GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        resolve.path_module_ids.insert(
            GtPathModuleId::new((0, 0).into(), "module".into()),
            "module/path".into(),
        );
        let mut context = Rst::convert_context_with_resolve(resolve);
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::new(
                    (0, 0).into(),
                    GtPathModuleId::new((0, 0).into(), "module".into()),
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
        let mut resolve = RsConvertResolve::default();
        resolve.path_module_ids.insert(
            GtPathModuleId::new((0, 0).into(), "module".into()),
            "module/path".into(),
        );
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::new(
                    (0, 0).into(),
                    GtPathModuleId::new((0, 0).into(), "module".into()),
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
            .convert(&mut Rst::convert_context_with_resolve(resolve))
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
        let mut resolve = RsConvertResolve::default();
        resolve.path_module_ids.insert(
            GtPathModuleId::new((0, 0).into(), "module".into()),
            "module/path".into(),
        );
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::new(
                    (0, 0).into(),
                    GtPathModuleId::new((0, 0).into(), "module".into()),
                    "./path/to/module".into()
                ),
                reference: GtIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut Rst::convert_context_with_resolve(resolve))
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
