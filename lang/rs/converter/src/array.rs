use genotype_lang_rs_tree::*;
use genotype_parser::tree::array::GTArray;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSVec> for GTArray {
    fn convert(&self, context: &mut RSConvertContext) -> RSVec {
        RSVec {
            descriptor: self.descriptor.convert(context),
        }
    }
}

#[cfg(test)]
mod tests {
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
            .convert(&mut RSConvertContext::empty("module".into())),
            RSVec {
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean)
            }
        );
    }
}
