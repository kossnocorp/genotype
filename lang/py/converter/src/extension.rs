use genotype_lang_py_tree::{definition::PYDefinition, PYExtension};
use genotype_parser::tree::GTExtension;

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYExtension> for GTExtension {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYExtension
    where
        HoistFn: Fn(PYDefinition),
    {
        PYExtension {
            reference: self.reference.convert(resolve, hoist),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use genotype_parser::GTIdentifier;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            PYExtension {
                reference: PYReference::new("Name".into(), false)
            },
            GTExtension {
                span: (0, 0).into(),
                reference: GTIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&PYConvertResolve::new(), &|_| {}),
        );
    }
}
