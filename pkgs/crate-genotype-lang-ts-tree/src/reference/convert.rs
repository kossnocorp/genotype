use crate::prelude::internal::*;

impl TsConvert<TsReference> for GtReference {
    fn convert(&self, context: &mut TsConvertContext) -> TsReference {
        TsReference::new(
            self.identifier.convert(context),
            self.arguments
                .iter()
                .map(|argument| argument.descriptor.convert(context))
                .collect(),
            TsReferenceRel::Regular,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(Gt::reference_anon("Name")),
            @r#"
        TsReference(
          identifier: TsIdentifier("Name"),
          arguments: [],
          rel: Regular,
        )
        "#,
        );
    }

    #[test]
    fn test_convert_with_arguments() {
        assert_ron_snapshot!(
            convert_node(GtReference {
                arguments: vec![Gt::primitive_string().into()],
                ..Gt::reference_anon("Name")
            }),
            @r#"
        TsReference(
          identifier: TsIdentifier("Name"),
          arguments: [
            Primitive(String),
          ],
          rel: Regular,
        )
        "#,
        );
    }
}
