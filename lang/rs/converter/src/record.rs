use genotype_lang_rs_tree::{RSContext, RSDependency, RSHashMap};
use genotype_parser::GTRecord;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSHashMap> for GTRecord {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSHashMap> {
        context.import(RSDependency::Std("collections".into()), "HashMap".into());
        Ok(RSHashMap {
            key: self.key.convert(context)?,
            descriptor: self.descriptor.convert(context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

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
            RSHashMap {
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
            RSHashMap {
                key: RSPrimitive::String.into(),
                descriptor: RSPrimitive::String.into(),
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Std("collections".into()), "HashMap".into()),]
        );
    }
}
