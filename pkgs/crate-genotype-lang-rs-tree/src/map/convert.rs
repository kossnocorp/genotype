use crate::prelude::internal::*;

impl RsConvert<RsMap> for GtRecord {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsMap> {
        context.push_import(RsUse::new(
            RsDependencyIdent::Std("collections".into()),
            "BTreeMap".into(),
        ));
        Ok(RsMap {
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
        RsMap(
          key: Primitive(String),
          descriptor: Primitive(String),
        )
        "
        );
    }

    #[test]
    fn test_convert_import() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            convert_node_with(
                Gt::record(Gt::record_key_string(), Gt::primitive_string()),
                &mut context
            ),
            @"
        RsMap(
          key: Primitive(String),
          descriptor: Primitive(String),
        )
        "
        );
        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          RsUse(
            dependency: Std("collections"),
            reference: Named([
              Name(RsIdentifier("BTreeMap")),
            ]),
          ),
        ]
        "#
        );
    }
}
