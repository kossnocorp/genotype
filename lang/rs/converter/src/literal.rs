use genotype_lang_rs_tree::{RSAttribute, RSContext, RSDependency, RSStruct};
use genotype_parser::tree::GTLiteral;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSStruct> for GTLiteral {
    fn convert(&self, context: &mut RSConvertContext) -> RSStruct {
        context.import(RSDependency::Runtime, "literal".into());

        let doc = context.consume_doc();
        let name = if let Some(name) = context.claim_alias() {
            name
        } else {
            context.name_child(&self.to_string())
        };

        let literal = render_literal(self);

        RSStruct {
            doc,
            attributes: vec![RSAttribute(format!("literal({literal})"))],
            name,
            fields: vec![].into(),
        }
    }
}

fn render_literal(literal: &GTLiteral) -> String {
    match literal {
        GTLiteral::Boolean(_, value) => value.to_string(),
        GTLiteral::Integer(_, value) => value.to_string(),
        GTLiteral::Float(_, value) => GTLiteral::render_float(&value),
        GTLiteral::String(_, value) => GTLiteral::render_string(&value),
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use pretty_assertions::assert_eq;

    use crate::context::{naming::RSContextParent, RSConvertContext};

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), true).convert(&mut RSConvertContext::default()),
            RSStruct {
                doc: None,
                attributes: vec![RSAttribute("literal(true)".into())],
                name: "True".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_name_from_alias() {
        let mut context = RSConvertContext::default();
        context.enter_parent(RSContextParent::Alias("Version".into()));
        assert_eq!(
            GTLiteral::Integer((0, 0).into(), 1).convert(&mut context),
            RSStruct {
                doc: None,
                attributes: vec![RSAttribute("literal(1)".into())],
                name: "Version".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_name_from_parents() {
        let mut context = RSConvertContext::default();
        context.enter_parent(RSContextParent::Definition("User".into()));
        context.enter_parent(RSContextParent::Property("v".into()));
        assert_eq!(
            GTLiteral::Integer((0, 0).into(), 1).convert(&mut context),
            RSStruct {
                doc: None,
                attributes: vec![RSAttribute("literal(1)".into())],
                name: "UserV1".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_import() {
        let mut context = Default::default();
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), false).convert(&mut context),
            RSStruct {
                doc: None,
                attributes: vec![RSAttribute("literal(false)".into())],
                name: "False".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Runtime, "literal".into())]
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = RSConvertContext::default();
        context.provide_doc(Some("Hello, world!".into()));
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), false).convert(&mut context),
            RSStruct {
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute("literal(false)".into())],
                name: "False".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }
}
