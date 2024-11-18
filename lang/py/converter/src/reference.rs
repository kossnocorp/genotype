use genotype_lang_py_tree::reference::PYReference;
use genotype_parser::tree::reference::GTReference;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYReference> for GTReference {
    fn convert(&self, context: &mut PYConvertContext) -> PYReference {
        let identifier = self.2.convert(context);
        let forward = context.is_forward_identifier(&identifier, &self.2);
        PYReference::new(identifier, forward)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use genotype_parser::{GTAliasId, GTIdentifier, GTReferenceAliasId};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        let mut context = PYConvertContext::default();
        context.push_defined(&"Name".into());
        assert_eq!(
            PYReference::new("Name".into(), false),
            GTReference(
                (0, 0).into(),
                GTReferenceAliasId::Resolved(GTAliasId("module".into(), "Name".into())),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut context),
        );
    }

    #[test]
    fn test_convert_forward() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            PYReference::new("Name".into(), true),
            GTReference(
                (0, 0).into(),
                GTReferenceAliasId::Resolved(GTAliasId("module".into(), "Name".into())),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut context),
        );
    }
}
