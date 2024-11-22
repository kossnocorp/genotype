use genotype_lang_ts_tree::reference::TSReference;
use genotype_parser::tree::reference::GTReference;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSReference> for GTReference {
    fn convert(&self, context: &mut TSConvertContext) -> TSReference {
        TSReference(self.identifier.convert(context))
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::{GTDefinitionId, GTIdentifier, GTReferenceDefinitionId, GTReferenceId};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSReference("Name".into()),
            GTReference {
                span: (0, 0).into(),
                id: GTReferenceId("module".into(), (0, 0).into()),
                definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                    "module".into(),
                    "Name".into()
                )),
                identifier: GTIdentifier::new((0, 0).into(), "Name".into())
            }
            .convert(&mut Default::default()),
        );
    }
}
