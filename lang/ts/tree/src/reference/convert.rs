use crate::prelude::internal::*;

impl TSConvert<TSReference> for GTReference {
    fn convert(&self, context: &mut TSConvertContext) -> TSReference {
        TSReference(self.identifier.convert(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
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
            .convert(&mut Default::default()),
            @r#"TSReference(TSIdentifier("Name"))"#,
        );
    }
}
