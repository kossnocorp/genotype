use genotype_lang_rs_tree::reference::RSReference;
use genotype_parser::tree::reference::GTReference;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSReference> for GTReference {
    fn convert(&self, context: &mut RSConvertContext) -> RSReference {
        let identifier = self.1.convert(context);
        let forward = context.is_forward_identifier(&identifier, &self.1);
        RSReference::new(identifier)
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
        let mut context = RSConvertContext::default();
        context.push_defined(&"Name".into());
        assert_eq!(
            RSReference::new("Name".into()),
            GTReference(
                (0, 0).into(),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut context),
        );
    }

    #[test]
    fn test_convert_forward() {
        let mut context = RSConvertContext::default();
        assert_eq!(
            RSReference::new("Name".into()),
            GTReference(
                (0, 0).into(),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut context),
        );
    }
}
