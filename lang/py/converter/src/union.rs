use genotype_lang_py_tree::{definition::PYDefinition, union::PYUnion};
use genotype_parser::tree::union::GTUnion;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYUnion> for GTUnion {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYUnion
    where
        HoistFn: Fn(PYDefinition),
    {
        PYUnion {
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
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            }
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::Boolean),
                    PYDescriptor::Primitive(PYPrimitive::String),
                ]
            }
        );
    }
}
