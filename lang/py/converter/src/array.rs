use genotype_lang_py_tree::*;
use genotype_parser::tree::array::GTArray;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYList> for GTArray {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYList
    where
        HoistFn: Fn(PYDefinition),
    {
        PYList {
            descriptor: self.descriptor.convert(resolve, hoist),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean)
            }
        );
    }
}
