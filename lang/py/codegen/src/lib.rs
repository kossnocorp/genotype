use genotype_lang_core_codegen::GtlCodegen;
use genotype_lang_core_tree::{indent::GTIndent, render::GTRenderModule};
use genotype_lang_py_converter::{context::PYConvertContext, convert::PYConvert};
use genotype_lang_py_tree::*;
use genotype_parser::*;
use miette::Result;

pub struct PyCodegen {}

impl PyCodegen {
    fn render_hoisted(context: &mut PYConvertContext) -> Option<String> {
        let hoisted = context.drain_hoisted();
        if hoisted.is_empty() {
            None
        } else {
            Some(PYModule::join_definitions(
                hoisted
                    .iter()
                    .map(|definition| definition.render(&py_indent(), &Default::default()))
                    .collect(),
            ))
        }
    }
}

impl GtlCodegen for PyCodegen {
    fn indent() -> GTIndent<'static> {
        py_indent()
    }

    fn render_descriptor(descriptor: &GTDescriptor) -> Result<(String, Option<String>)> {
        let mut context = PYConvertContext::default();
        let converted = descriptor.convert(&mut context);
        let rendered = converted.render(&py_indent(), &Default::default());
        let hoisted_rendered = Self::render_hoisted(&mut context);
        Ok((rendered, hoisted_rendered))
    }

    fn render_alias(alias: &GTAlias) -> Result<String> {
        let mut rendered = vec![];

        let mut context = PYConvertContext::default();
        let converted = alias.convert(&mut context);

        let rendered_alias = converted.render(&py_indent(), &Default::default());
        rendered.push(rendered_alias);

        if let Some(hoisted_rendered) = Self::render_hoisted(&mut context) {
            rendered.push(hoisted_rendered);
        }

        Ok(PYModule::join_definitions(rendered))
    }
}

#[cfg(test)]
mod tespy {
    use super::*;
    use genotype_parser::{GTAlias, GTDefinitionId, GTIdentifier, GTLiteral, GTModuleId, GTUnion};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_descriptor() {
        let literal = GTDescriptor::Literal(GTLiteral::Boolean(Default::default(), true));
        assert_eq!(
            PyCodegen::render_descriptor(&literal).unwrap(),
            ("Literal[True]".into(), None)
        );
    }

    #[test]
    fn test_render_descriptor_with_hoisted() {
        let alias = GTDescriptor::Alias(Box::new(GTAlias {
            id: GTDefinitionId(GTModuleId("module".into()), "Hello".into()),
            span: Default::default(),
            doc: None,
            attributes: vec![],
            name: GTIdentifier::new(Default::default(), "Hello".into()),
            descriptor: GTDescriptor::Literal(GTLiteral::Boolean(Default::default(), true)),
        }));
        assert_eq!(
            PyCodegen::render_descriptor(&alias).unwrap(),
            ("Hello".into(), Some("type Hello = Literal[True]".into()))
        );
    }

    #[test]
    fn test_render_descriptor_with_multiple_hoisted() {
        let union = GTDescriptor::Union(GTUnion {
            span: Default::default(),
            descriptors: vec![
                GTDescriptor::Alias(Box::new(GTAlias {
                    id: GTDefinitionId(GTModuleId("module".into()), "Hello".into()),
                    span: Default::default(),
                    doc: None,
                    attributes: vec![],
                    name: GTIdentifier::new(Default::default(), "Hello".into()),
                    descriptor: GTDescriptor::Literal(GTLiteral::Boolean(Default::default(), true)),
                })),
                GTDescriptor::Alias(Box::new(GTAlias {
                    id: GTDefinitionId(GTModuleId("module".into()), "World".into()),
                    span: Default::default(),
                    doc: None,
                    attributes: vec![],
                    name: GTIdentifier::new(Default::default(), "World".into()),
                    descriptor: GTDescriptor::Literal(GTLiteral::String(
                        Default::default(),
                        "world".into(),
                    )),
                })),
            ],
        });
        assert_eq!(
            PyCodegen::render_descriptor(&union).unwrap(),
            (
                "Hello | World".into(),
                Some(
                    r#"type Hello = Literal[True]


type World = Literal["world"]"#
                        .into()
                )
            )
        );
    }

    #[test]
    fn test_render_alias() {
        let alias = GTAlias {
            span: Default::default(),
            id: GTDefinitionId(GTModuleId("module".into()), "Hi".into()),
            doc: None,
            attributes: vec![],
            name: GTIdentifier::new(Default::default(), "Hi".into()),
            descriptor: GTDescriptor::Union(GTUnion {
                span: Default::default(),
                descriptors: vec![
                    GTDescriptor::Alias(Box::new(GTAlias {
                        id: GTDefinitionId(GTModuleId("module".into()), "Hello".into()),
                        span: Default::default(),
                        doc: None,
                        attributes: vec![],
                        name: GTIdentifier::new(Default::default(), "Hello".into()),
                        descriptor: GTDescriptor::Literal(GTLiteral::Boolean(
                            Default::default(),
                            true,
                        )),
                    })),
                    GTDescriptor::Alias(Box::new(GTAlias {
                        id: GTDefinitionId(GTModuleId("module".into()), "World".into()),
                        span: Default::default(),
                        doc: None,
                        attributes: vec![],
                        name: GTIdentifier::new(Default::default(), "World".into()),
                        descriptor: GTDescriptor::Literal(GTLiteral::String(
                            Default::default(),
                            "world".into(),
                        )),
                    })),
                ],
            }),
        };
        assert_eq!(
            PyCodegen::render_alias(&alias).unwrap(),
            r#"type Hi = Hello | World


type Hello = Literal[True]


type World = Literal["world"]"#
        );
    }
}
