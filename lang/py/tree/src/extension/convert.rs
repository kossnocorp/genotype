use crate::prelude::internal::*;

impl PYConvert<PYExtension> for GTExtension {
    fn convert(&self, context: &mut PYConvertContext) -> PYExtension {
        PYExtension {
            reference: self.reference.convert(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTExtension {
                span: (0, 0).into(),
                reference: GTReference {
                    span: (0, 0).into(),
                    id: GTReferenceId("module".into(), (0, 0).into()),
                    definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                        "module".into(),
                        "Name".into()
                    )),
                    identifier: GTIdentifier::new((0, 0).into(), "Name".into()),
                }
                .into()
            }
            .convert(&mut PYConvertContext::default()),
            @r#"
        PYExtension(
          reference: PYReference(
            identifier: PYIdentifier("Name"),
            forward: true,
          ),
        )
        "#,
        );
    }
}
