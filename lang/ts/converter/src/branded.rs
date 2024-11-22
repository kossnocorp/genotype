use genotype_lang_ts_tree::{definition::TSDefinition, TSBranded};
use genotype_parser::GTBranded;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSBranded> for GTBranded {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSBranded
    where
        HoistFn: Fn(TSDefinition),
    {
        let name = self.name.convert(resolve, hoist);
        let primitive = self.primitive.convert(resolve, hoist);

        // [TODO] Hoist
        TSBranded {
            // [TODO]
            doc: None,
            name,
            primitive,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::resolve::TSConvertResolve;

    use super::*;
    use genotype_parser::{GTDefinitionId, GTIdentifier};

    // #[test]
    // fn test_convert() {
    //     assert_eq!(
    //         GTBranded::String(
    //             (0, 0).into(),
    //             GTDefinitionId("module".into(), "UserId".into()),
    //             GTIdentifier::new((0, 0).into(), "UserId".into())
    //         )
    //         .convert(&TSConvertResolve::new(), &|_| {}),
    //         TSBranded {
    //             doc: None,
    //             name: "UserId".into(),
    //             primitive: TSPrimitive::String,
    //         }
    //     );
    // }
}
