use genotype_lang_py_tree::PYExtension;
use genotype_parser::tree::GTExtension;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYExtension> for GTExtension {
    fn convert(&self, context: &mut PYConvertContext) -> PYExtension {
        PYExtension {
            reference: self.reference.convert(context),
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
                reference: PYReference::new("Name".into(), true)
            },
            GTExtension {
                span: (0, 0).into(),
                reference: GTIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut PYConvertContext::default()),
        );
    }
}
