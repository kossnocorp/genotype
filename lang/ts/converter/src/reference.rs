use genotype_lang_ts_tree::{definition::TSDefinition, reference::TSReference};
use genotype_parser::tree::reference::GTReference;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSReference> for GTReference {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSReference
    where
        HoistFn: Fn(TSDefinition),
    {
        TSReference(self.identifier.convert(resolve, hoist))
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
            .convert(&TSConvertResolve::new(), &|_| {}),
        );
    }
}
