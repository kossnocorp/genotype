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
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_to_py(GtFactory::array(GtFactory::primitive_boolean())),
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
            GtFactory::array(GtFactory::primitive_string())
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
