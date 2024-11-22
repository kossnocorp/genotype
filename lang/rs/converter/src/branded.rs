use genotype_lang_rs_tree::{
    RSContext, RSContextRenderDeriveMode, RSPrimitive, RSStruct, RSStructFields,
};
use genotype_parser::GTBranded;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSStruct> for GTBranded {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSStruct> {
        let doc = context.consume_doc();
        let name = self.name.convert(context)?;
        let id = context
            .consume_definition_id()
            .unwrap_or_else(|| context.build_definition_id(&name));
        let descriptor = self.primitive.convert(context)?.into();

        Ok(RSStruct {
            id,
            doc,
            attributes: vec![context
                .render_derive(RSContextRenderDeriveMode::Struct)
                .into()],
            name,
            fields: RSStructFields::Tuple(vec![descriptor]),
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use genotype_parser::{GTDefinitionId, GTLiteral};
    use pretty_assertions::assert_eq;

    use crate::context::{naming::RSContextParent, RSConvertContext};

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), true)
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "True".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(true)".into())],
                name: "True".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_name_from_alias() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));
        assert_eq!(
            GTLiteral::Integer((0, 0).into(), 1)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "Version".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(1)".into())],
                name: "Version".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_name_from_parents() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("User".into()));
        context.enter_parent(RSContextParent::Field("v".into()));
        assert_eq!(
            GTLiteral::Integer((0, 0).into(), 1)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "UserV1".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(1)".into())],
                name: "UserV1".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_import() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), false)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "False".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(false)".into())],
                name: "False".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Literals, "literal".into())]
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = RSConvertContext::empty("module".into());
        context.provide_doc(Some("Hello, world!".into()));
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), false)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "False".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute("literal(false)".into())],
                name: "False".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }
}
