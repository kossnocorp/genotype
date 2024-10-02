use genotype_lang_ts_tree::{definition::TSDefinition, reference::TSReference};
use genotype_parser::tree::reference::GTReference;

use crate::convert::TSConvert;

impl TSConvert<TSReference> for GTReference {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSReference
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            GTReference::Unresolved(name) => TSReference::Unresolved(name.convert(hoist)),

            GTReference::External(name, path) => {
                TSReference::External(name.convert(hoist), path.convert(hoist))
            }

            GTReference::Local(name) => TSReference::Local(name.convert(hoist)),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::*;

    #[test]
    fn test_convert_unresolved() {
        assert_eq!(
            TSReference::Unresolved("Name".into()),
            GTReference::Unresolved("Name".into()).convert(&|_| {}),
        );
    }

    #[test]
    fn test_convert_external() {
        assert_eq!(
            TSReference::External("Name".into(), TSPath::Unresolved("./path/to/module".into())),
            GTReference::External("Name".into(), GTPath("./path/to/module".into()))
                .convert(&|_| {}),
        );
    }

    #[test]
    fn test_convert_internal() {
        assert_eq!(
            TSReference::Local("Name".into()),
            GTReference::Local("Name".into()).convert(&|_| {}),
        );
    }
}
