use genotype_lang_ts_tree::{definition::TSDefinition, primitive::TSPrimitive, TSBranded};
use genotype_parser::GTBranded;

use crate::{convert::TSConvert, resolve::TSConvertResolve};

impl TSConvert<TSBranded> for GTBranded {
    fn convert<HoistFn>(&self, resolve: &TSConvertResolve, hoist: &HoistFn) -> TSBranded
    where
        HoistFn: Fn(TSDefinition),
    {
        let (identifier, primitive) = match self {
            GTBranded::Boolean(_, _, identifier) => (identifier, TSPrimitive::Boolean),
            GTBranded::String(_, _, identifier) => (identifier, TSPrimitive::String),
            GTBranded::Int(_, _, identifier) => (identifier, TSPrimitive::Number),
            GTBranded::Float(_, _, identifier) => (identifier, TSPrimitive::Number),
            GTBranded::Null(_, _, identifier) => (identifier, TSPrimitive::Null),
        };

        let name = self.convert(resolve, hoist);

        TSBranded {
            doc,
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

    #[test]
    fn test_convert() {
        assert_eq!(
            GTBranded::String(
                (0, 0).into(),
                GTDefinitionId("module".into(), "UserId".into()),
                GTIdentifier::new((0, 0).into(), "UserId".into())
            )
            .convert(&TSConvertResolve::new(), &|_| {}),
            TSBranded {
                doc: None,
                name: "UserId".into(),
                primitive: TSPrimitive::String,
            }
        );
    }
}
