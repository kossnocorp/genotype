use genotype_lang_py_tree::*;
use genotype_parser::tree::array::GTArray;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYList> for GTArray {
    fn convert(&self, context: &mut PYConvertContext) -> PYList {
        PYList {
            descriptor: self.descriptor.convert(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::context::PYConvertContext;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut PYConvertContext::default()),
            PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean)
            }
        );
    }
}
