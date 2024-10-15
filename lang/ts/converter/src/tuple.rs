use genotype_lang_ts_tree::{definition::TSDefinition, tuple::TSTuple};
use genotype_parser::tree::tuple::GTTuple;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSTuple> for GTTuple {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSTuple
    where
        HoistFn: Fn(TSDefinition),
    {
        TSTuple {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(resolve, hoist))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::resolve::TSConvertResolve;

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
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSTuple {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Boolean),
                    TSDescriptor::Primitive(TSPrimitive::String),
                ]
            }
        );
    }
}
