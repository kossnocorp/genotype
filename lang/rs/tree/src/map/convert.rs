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
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTRecord {
                span: (0, 0).into(),
                key: GTRecordKey::String((0, 0).into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
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
            GTRecord {
                span: (0, 0).into(),
                key: GTRecordKey::String((0, 0).into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context)
            .unwrap(),
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
