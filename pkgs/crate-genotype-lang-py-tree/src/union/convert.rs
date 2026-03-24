use crate::prelude::internal::*;

impl PYConvert<PYUnion> for GTUnion {
    fn convert(&self, context: &mut PYConvertContext) -> PYUnion {
        PYUnion {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(context))
                .collect(),
            discriminator: None,
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(Gt::union(vec![
                Gt::primitive_boolean().into(),
                Gt::primitive_string().into(),
            ])),
            @"
        PYUnion(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
          discriminator: None,
        )
        "
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::new(
            PYConvertResolve::default(),
            PyConfig {
                lang: PyConfigLang::new(PYVersion::Legacy),
                ..Default::default()
            },
        );
        assert_ron_snapshot!(
            convert_node_with(Gt::union(vec![
                Gt::primitive_string().into(),
            ]), &mut context),
            @"
        PYUnion(
          descriptors: [
            Primitive(String),
          ],
          discriminator: None,
        )
        "
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PYIdentifier("Union")),
        ]
        "#
        );
    }
}
