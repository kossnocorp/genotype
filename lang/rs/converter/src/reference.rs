use genotype_lang_rs_tree::reference::RSReference;
use genotype_parser::{tree::reference::GTReference, GTReferenceDefinitionId};
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert, error::RSConverterError};

impl RSConvert<RSReference> for GTReference {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSReference> {
        let identifier = self.identifier.convert(context)?;
        let definition_id = match &self.definition_id {
            GTReferenceDefinitionId::Resolved(id) => id.clone(),
            GTReferenceDefinitionId::Unresolved => {
                return Err(RSConverterError::UnresolvedReference(self.span.clone()).into())
            }
        };

        Ok(RSReference {
            id: self.id.clone(),
            identifier,
            definition_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use genotype_parser::{GTDefinitionId, GTIdentifier, GTReferenceDefinitionId, GTReferenceId};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        let mut context = RSConvertContext::empty("module".into());
        context.push_defined(&"Name".into());
        assert_eq!(
            RSReference {
                id: GTReferenceId("module".into(), (1, 8).into()),
                identifier: "Name".into(),
                definition_id: GTDefinitionId("module".into(), "Name".into())
            },
            GTReference {
                span: (0, 0).into(),
                id: GTReferenceId("module".into(), (1, 8).into()),
                definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                    "module".into(),
                    "Name".into()
                )),
                identifier: GTIdentifier::new((0, 0).into(), "Name".into())
            }
            .convert(&mut context)
            .unwrap(),
        );
    }
}
