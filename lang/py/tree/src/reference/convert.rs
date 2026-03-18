use crate::prelude::internal::*;

impl PYConvert<PYReference> for GTReference {
    fn convert(&self, context: &mut PYConvertContext) -> PYReference {
        let identifier = self.identifier.convert(context);
        let forward = context.is_forward_identifier(&identifier, &self.identifier);
        PYReference::new(identifier, forward)
    }
}

impl PYConvert<PYReference> for GTInlineImport {
    fn convert(&self, context: &mut PYConvertContext) -> PYReference {
        let name = self.name.convert(context);
        let path = self.path.convert(context);
        context.add_import(PYDependencyIdent::Path(path), name.clone());
        PYReference::new(name, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_reference() {
        let mut context = PYConvertContext::default();
        context.push_defined(&"Name".into());
        assert_ron_snapshot!(
            GTReference {
                span: (0, 0).into(),
                id: GTReferenceId("module".into(), (0, 0).into()),
                definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                    "module".into(),
                    "Name".into()
                )),
                identifier: GTIdentifier::new((0, 0).into(), "Name".into())
            }
            .convert(&mut context),
            @r#"
        PYReference(
          identifier: PYIdentifier("Name"),
          forward: false,
        )
        "#,
        );
    }

    #[test]
    fn test_convert_reference_forward() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            GTReference {
                span: (0, 0).into(),
                id: GTReferenceId("module".into(), (0, 0).into()),
                definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                    "module".into(),
                    "Name".into()
                )),
                identifier: GTIdentifier::new((0, 0).into(), "Name".into())
            }
            .convert(&mut context),
            @r#"
        PYReference(
          identifier: PYIdentifier("Name"),
          forward: true,
        )
        "#,
        );
    }

    #[test]
    fn test_convert_inline_import() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
            }
            .convert(&mut context),
            @r#"
        PYReference(
          identifier: PYIdentifier("Name"),
          forward: false,
        )
        "#,
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Path(PYPath(".path.to.module")), PYIdentifier("Name")),
        ]
        "#
        );
    }
}
