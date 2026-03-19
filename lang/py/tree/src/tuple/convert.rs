use crate::prelude::internal::*;

impl PYConvert<PYTuple> for GTTuple {
    fn convert(&self, context: &mut PYConvertContext) -> PYTuple {
        PYTuple {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(context))
                .collect(),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTTuple {
                span: (0, 0).into(),
                descriptors: vec![
                    GtFactory::primitive_boolean().into(),
                    GtFactory::primitive_string().into(),
                ]
            }
            .convert(&mut PYConvertContext::default()),
            @"
        PYTuple(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
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
            GTTuple {
                span: (0, 0).into(),
                descriptors: vec![GtFactory::primitive_string().into()],
            }
            .convert(&mut context),
            @"
        PYTuple(
          descriptors: [
            Primitive(String),
          ],
        )
        "
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PYIdentifier("Tuple")),
        ]
        "#
        );
    }
}
