use genotype_lang_rs_tree::*;
use genotype_parser::tree::array::GTArray;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSList> for GTArray {
    fn convert(&self, context: &mut RSConvertContext) -> RSList {
        RSList {
            descriptor: self.descriptor.convert(context),
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_config::{RSLangConfig, RSVersion};
    use genotype_lang_rs_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::context::RSConvertContext;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut RSConvertContext::default()),
            RSList {
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean)
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context =
            RSConvertContext::new(Default::default(), RSLangConfig::new(RSVersion::Legacy));
        assert_eq!(
            GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context),
            RSList {
                descriptor: RSPrimitive::String.into(),
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Typing, "List".into())]
        );
    }
}
