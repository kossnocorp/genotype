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
        context.add_import(PYDependencyIdent::Local(path), name.clone());
        PYReference::new(name, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_reference() {
        let mut context = PYConvertContext::default();
        context.push_defined(&"Name".into());
        assert_eq!(
            PYReference::new("Name".into(), false),
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
        );
    }

    #[test]
    fn test_convert_reference_forward() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            PYReference::new("Name".into(), true),
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
        );
    }

    #[test]
    fn test_convert_inline_import() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
            }
            .convert(&mut context),
            PYReference::new("Name".into(), false),
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(
                PYDependencyIdent::Local(".path.to.module".into()),
                "Name".into()
            )]
        );
    }
}
