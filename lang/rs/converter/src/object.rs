use genotype_lang_rs_tree::*;
use genotype_parser::*;

use crate::{
    context::{naming::RSContextParent, RSConvertContext},
    convert::RSConvert,
};

impl RSConvert<RSStruct> for GTObject {
    fn convert(&self, context: &mut RSConvertContext) -> RSStruct {
        let name = match &self.name {
            GTObjectName::Named(identifier) => identifier.convert(context),
            GTObjectName::Alias(identifier, _) => identifier.convert(context),
            _ => panic!("Invalid object name"),
        };
        context.enter_parent(RSContextParent::Definition(name.clone()));

        let doc = context.consume_doc();
        let extensions = self.extensions.iter().map(|e| e.convert(context)).collect();
        let properties = self.properties.iter().map(|p| p.convert(context)).collect();

        let r#struct = RSStruct {
            doc,
            // [TODO]
            attributes: vec![],
            name,
            extensions,
            properties,
        }
        .resolve(context);

        context.exit_parent();
        r#struct
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: GTPrimitive::Int((0, 0).into()).into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut RSConvertContext::default()),
            RSStruct {
                doc: None,
                attributes: vec![],
                name: "Person".into(),
                extensions: vec![],
                properties: vec![
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String).into(),
                    },
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSOption::new(RSDescriptor::Primitive(RSPrimitive::Int)).into(),
                    }
                ],
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = RSConvertContext::default();
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![]
            }
            .convert(&mut context),
            RSStruct {
                doc: None,
                attributes: vec![],
                name: "Person".into(),
                extensions: vec![],
                properties: vec![],
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Runtime, "Model".into())]
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = RSConvertContext::default();
        context.provide_doc(Some("Hello, world!".into()));
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![],
            }
            .convert(&mut context),
            RSStruct {
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Person".into(),
                extensions: vec![],
                properties: vec![],
            }
        );
    }
}
