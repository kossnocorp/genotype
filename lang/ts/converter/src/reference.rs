use genotype_lang_ts_tree::{definition::TSDefinition, reference::TSReference};
use genotype_parser::tree::reference::GTReference;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSReference> for GTReference {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSReference
    where
        HoistFn: Fn(TSDefinition),
    {
        TSReference(self.2.convert(resolve, hoist))
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use genotype_parser::{GTDefinitionId, GTIdentifier, GTReferenceDefinitionId};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            TSReference("Name".into()),
            GTReference(
                (0, 0).into(),
                GTReferenceDefinitionId::Resolved(GTDefinitionId("module".into(), "Name".into())),
                GTIdentifier::new((0, 0).into(), "Name".into())
            )
            .convert(&TSConvertResolve::new(), &|_| {}),
        );
    }
}
