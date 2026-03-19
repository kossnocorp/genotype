use crate::prelude::internal::*;

impl PYConvert<PYList> for GTArray {
    fn convert(&self, context: &mut PYConvertContext) -> PYList {
        PYList {
            descriptor: self.descriptor.convert(context),
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
            GTArray {
                span: (0, 0).into(),
                descriptor: GtFactory::primitive_boolean().into(),
            }
            .convert(&mut PYConvertContext::default()),
            @"
        PYList(
          descriptor: Primitive(Boolean),
        )
        "
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::new(
            Default::default(),
            PyConfig {
                lang: PyConfigLang::new(PYVersion::Legacy),
                ..Default::default()
            },
        );
        assert_ron_snapshot!(
            GTArray {
                span: (0, 0).into(),
                descriptor: GtFactory::primitive_string().into(),
            }
            .convert(&mut context),
            @"
        PYList(
          descriptor: Primitive(String),
        )
        "
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PYIdentifier("List")),
        ]
        "#
        );
    }
}
