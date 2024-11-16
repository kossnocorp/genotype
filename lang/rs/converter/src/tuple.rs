use genotype_lang_rs_tree::tuple::RSTuple;
use genotype_parser::tree::tuple::GTTuple;

use crate::{
    context::{naming::RSContextParent, RSConvertContext},
    convert::RSConvert,
};

impl RSConvert<RSTuple> for GTTuple {
    fn convert(&self, context: &mut RSConvertContext) -> RSTuple {
        context.enter_parent(RSContextParent::Anonymous);

        let descriptors = self
            .descriptors
            .iter()
            .map(|descriptor| descriptor.convert(context))
            .collect();
        let tuple = RSTuple { descriptors };

        context.exit_parent();
        tuple
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
            GTTuple {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            }
            .convert(&mut RSConvertContext::default()),
            RSTuple {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::Boolean),
                    RSDescriptor::Primitive(RSPrimitive::String),
                ]
            }
        );
    }
}
