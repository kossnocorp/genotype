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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTRecord {
                span: (0, 0).into(),
                key: GTRecordKey::String((0, 0).into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSMap {
                key: RSPrimitive::String.into(),
                descriptor: RSPrimitive::String.into(),
            }
        );
    }

    #[test]
    fn test_convert_import() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTRecord {
                span: (0, 0).into(),
                key: GTRecordKey::String((0, 0).into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context)
            .unwrap(),
            RSMap {
                key: RSPrimitive::String.into(),
                descriptor: RSPrimitive::String.into(),
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(
                RSDependencyIdent::Std("collections".into()),
                "BTreeMap".into()
            ),]
        );
    }
}
