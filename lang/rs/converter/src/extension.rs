use genotype_lang_rs_tree::RSExtension;
use genotype_parser::tree::GTExtension;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSExtension> for GTExtension {
    fn convert(&self, context: &mut RSConvertContext) -> RSExtension {
        RSExtension {
            reference: self.reference.convert(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use genotype_parser::GTIdentifier;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            RSExtension {
                reference: RSReference::new("Name".into())
            },
            GTExtension {
                span: (0, 0).into(),
                reference: GTIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut RSConvertContext::default()),
        );
    }
}
