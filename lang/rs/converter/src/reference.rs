use genotype_lang_rs_tree::reference::RSReference;
use genotype_parser::{tree::reference::GTReference, GTReferenceDefinitionId};
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert, error::RSConverterError};

impl RSConvert<RSReference> for GTReference {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSReference> {
        let identifier = self.2.convert(context)?;
        let definition_id = match &self.1 {
            GTReferenceDefinitionId::Resolved(id) => id.clone(),
            GTReferenceDefinitionId::Unresolved => {
                return Err(RSConverterError::UnresolvedReference(self.0.clone()).into())
            }
        };

        Ok(RSReference {
            identifier,
            definition_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use genotype_parser::{GTDefinitionId, GTIdentifier, GTReferenceDefinitionId};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        let mut context = RSConvertContext::empty("module".into());
        context.push_defined(&"Name".into());
        assert_eq!(
            RSReference::new(
                "Name".into(),
                GTDefinitionId("module".into(), "Name".into())
            ),
            GTReference(
                (0, 0).into(),
                GTReferenceDefinitionId::Resolved(GTDefinitionId("module".into(), "Name".into())),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&mut context)
            .unwrap(),
        );
    }
}
