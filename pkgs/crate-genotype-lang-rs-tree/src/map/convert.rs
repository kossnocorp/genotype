use crate::prelude::internal::*;

impl RSConvert<RSMap> for GTRecord {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSMap> {
        context.add_import(
            RSDependencyIdent::Std("collections".into()),
            "BTreeMap".into(),
        );
        Ok(RSMap {
            key: self.key.convert(context)?,
            descriptor: self.descriptor.convert(context)?,
        })
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
            convert_node(
                Gt::record(Gt::record_key_string(), Gt::primitive_string())
            ),
            @"
        RSMap(
          key: Primitive(String),
          descriptor: Primitive(String),
        )
        "
        );
    }

    #[test]
    fn test_convert_import() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            convert_node_with(
                Gt::record(Gt::record_key_string(), Gt::primitive_string()),
                &mut context
            ),
            @"
        RSMap(
          key: Primitive(String),
          descriptor: Primitive(String),
        )
        "
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Std("collections"), RSIdentifier("BTreeMap")),
        ]
        "#
        );
    }
}
