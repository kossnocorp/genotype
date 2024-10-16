use genotype_lang_py_tree::{definition::PYDefinition, tuple::PYTuple};
use genotype_parser::tree::tuple::GTTuple;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYTuple> for GTTuple {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYTuple
    where
        HoistFn: Fn(PYDefinition),
    {
        PYTuple {
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
    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::resolve::PYConvertResolve;

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
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYTuple {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::Boolean),
                    PYDescriptor::Primitive(PYPrimitive::String),
                ]
            }
        );
    }
}
