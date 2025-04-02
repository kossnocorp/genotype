use genotype_lang_ts_tree::TSBranded;
use genotype_parser::GTBranded;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSBranded> for GTBranded {
    fn convert(&self, context: &mut TSConvertContext) -> TSBranded {
        let doc = context.consume_doc();
        let name = self.name.convert(context);
        let primitive = self.primitive.convert(context);

        TSBranded {
            doc,
            name,
            primitive,
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::TSPrimitive;
    use pretty_assertions::assert_eq;

    use crate::resolve::TSConvertResolve;

    use super::*;
    use genotype_parser::{GTDefinitionId, GTIdentifier, GTPrimitive};

    #[test]
    fn test_convert() {
        assert_eq!(
            GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut Default::default()),
            TSBranded {
                doc: None,
                name: "UserId".into(),
                primitive: TSPrimitive::String,
            }
        );
    }

    #[test]
    fn test_doc() {
        let mut context = TSConvertContext::new(TSConvertResolve::new(), None);
        context.provide_doc(Some("This is a user ID.".into()));
        assert_eq!(
            GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context),
            TSBranded {
                doc: Some("This is a user ID.".into()),
                name: "UserId".into(),
                primitive: TSPrimitive::String,
            }
        );
    }
}
