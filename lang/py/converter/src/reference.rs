use genotype_lang_py_tree::reference::PYReference;
use genotype_parser::tree::reference::GTReference;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYReference> for GTReference {
    fn convert(&self, context: &mut PYConvertContext) -> PYReference {
        let identifier = self.identifier.convert(context);
        let forward = context.is_forward_identifier(&identifier, &self.identifier);
        PYReference::new(identifier, forward)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use genotype_parser::{GTDefinitionId, GTIdentifier, GTReferenceDefinitionId, GTReferenceId};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        let mut context = PYConvertContext::default();
        context.push_defined(&"Name".into());
        assert_eq!(
            PYReference::new("Name".into(), false),
            GTReference {
                span: (0, 0).into(),
                id: GTReferenceId("module".into(), (0, 0).into()),
                definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                    "module".into(),
                    "Name".into()
                )),
                identifier: GTIdentifier::new((0, 0).into(), "Name".into())
            }
            .convert(&mut context),
        );
    }

    #[test]
    fn test_convert_forward() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            PYReference::new("Name".into(), true),
            GTReference {
                span: (0, 0).into(),
                id: GTReferenceId("module".into(), (0, 0).into()),
                definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                    "module".into(),
                    "Name".into()
                )),
                identifier: GTIdentifier::new((0, 0).into(), "Name".into())
            }
            .convert(&mut context),
        );
    }
}
