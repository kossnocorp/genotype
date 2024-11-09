use genotype_lang_rs_tree::*;
use genotype_parser::tree::descriptor::GTDescriptor;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSDescriptor> for GTDescriptor {
    fn convert(&self, context: &mut RSConvertContext) -> RSDescriptor {
        match self {
            GTDescriptor::Alias(alias) => context.hoist(|context| alias.convert(context)).into(),

            GTDescriptor::Array(array) => array.convert(context).into(),

            GTDescriptor::InlineImport(import) => {
                let reference = import.convert(context);
                context.track_reference(&reference);
                reference.into()
            }

            GTDescriptor::Literal(literal) => literal.convert(context).into(),

            GTDescriptor::Object(object) => context
                .hoist(|context| object.convert(context).into())
                .into(),

            GTDescriptor::Primitive(primitive) => primitive.convert(context).into(),

            GTDescriptor::Record(record) => record.convert(context).into(),

            GTDescriptor::Reference(name) => {
                let reference = name.convert(context);
                context.track_reference(&reference);
                reference.into()
            }

            GTDescriptor::Tuple(tuple) => tuple.convert(context).into(),

            GTDescriptor::Union(union) => union.convert(context).into(),

            GTDescriptor::Any(any) => any.convert(context).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_convert_alias() {
        let mut context = RSConvertContext::default();
        assert_eq!(
            GTDescriptor::Alias(Box::new(GTAlias {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut context),
            RSReference::new("Name".into()).into()
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![RSDefinition::Alias(RSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean),
                references: vec![],
            })]
        );
    }

    #[test]
    fn test_convert_array() {
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }))
            .convert(&mut RSConvertContext::default()),
            RSDescriptor::List(Box::new(RSList {
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean)
            }))
        );
    }

    #[test]
    fn test_convert_inline_import() {
        let mut context = RSConvertContext::default();
        assert_eq!(
            GTDescriptor::InlineImport(GTInlineImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                name: GTIdentifier::new((0, 0).into(), "Name".into())
            })
            .convert(&mut context),
            RSDescriptor::Reference(RSReference::new("Name".into()))
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(
                RSDependency::Local("self::path::to::module".into()),
                "Name".into()
            ),]
        );
    }

    #[test]
    fn test_convert_object() {
        let mut context = RSConvertContext::default();
        assert_eq!(
            GTDescriptor::Object(GTObject {
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
                ],
            })
            .convert(&mut context),
            RSDescriptor::Reference(RSReference::new("Person".into()))
        );
        let hoisted = context.drain_hoisted();
        assert_eq!(
            hoisted,
            vec![RSDefinition::Class(RSClass {
                doc: None,
                name: "Person".into(),
                extensions: vec![],
                properties: vec![
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSPrimitive::String.into(),
                    },
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSOption::new(RSPrimitive::Int.into()).into(),
                    }
                ],
                references: vec![],
            })]
        );
    }

    #[test]
    fn test_convert_primitive() {
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean((0, 0).into()))
                .convert(&mut RSConvertContext::default()),
            RSDescriptor::Primitive(RSPrimitive::Boolean)
        );
    }

    #[test]
    fn test_convert_reference() {
        assert_eq!(
            GTDescriptor::Reference(GTIdentifier::new((0, 0).into(), "Name".into()).into())
                .convert(&mut RSConvertContext::default()),
            RSReference::new("Name".into()).into()
        );
    }

    #[test]
    fn test_convert_tuple() {
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut RSConvertContext::default()),
            RSDescriptor::Tuple(RSTuple {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::Boolean),
                    RSDescriptor::Primitive(RSPrimitive::String),
                ]
            })
        );
    }

    #[test]
    fn test_convert_union() {
        assert_eq!(
            GTDescriptor::Union(GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            })
            .convert(&mut RSConvertContext::default()),
            RSDescriptor::Union(RSUnion {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::Boolean),
                    RSDescriptor::Primitive(RSPrimitive::String),
                ],
                discriminator: None
            })
        );
    }
}
