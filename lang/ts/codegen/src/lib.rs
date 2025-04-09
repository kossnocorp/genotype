use genotype_lang_core_codegen::GtlCodegen;
use genotype_lang_core_tree::{
    indent::GTIndent,
    render::{GTRender, GTRenderModule},
};
use genotype_lang_ts_converter::{context::TSConvertContext, convert::TSConvert};
use genotype_lang_ts_tree::*;
use genotype_parser::*;
use miette::Result;

pub struct TsCodegen {}

impl TsCodegen {
    fn render_hoisted(context: &mut TSConvertContext) -> Option<String> {
        let hoisted = context.drain_hoisted();
        if hoisted.is_empty() {
            None
        } else {
            Some(TSModule::join_definitions(
                hoisted
                    .iter()
                    .map(|definition| definition.render(&ts_indent()))
                    .collect(),
            ))
        }
    }
}

impl GtlCodegen for TsCodegen {
    fn indent() -> GTIndent<'static> {
        ts_indent()
    }

    fn render_descriptor(descriptor: &GTDescriptor) -> Result<(String, Option<String>)> {
        let mut context = TSConvertContext::default();
        let converted = descriptor.convert(&mut context);
        let rendered = converted.render(&ts_indent());
        let hoisted_rendered = Self::render_hoisted(&mut context);
        Ok((rendered, hoisted_rendered))
    }

    fn render_alias(alias: &GTAlias) -> Result<String> {
        let mut rendered = vec![];

        let mut context = TSConvertContext::default();
        let converted = alias.convert(&mut context);

        let rendered_alias = converted.render(&ts_indent());
        rendered.push(rendered_alias);

        if let Some(hoisted_rendered) = Self::render_hoisted(&mut context) {
            rendered.push(hoisted_rendered);
        }

        Ok(TSModule::join_definitions(rendered))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::{GTAlias, GTDefinitionId, GTIdentifier, GTLiteral, GTModuleId, GTUnion};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_descriptor() {
        let literal = GTDescriptor::Literal(GTLiteral::Boolean(Default::default(), true));
        assert_eq!(
            TsCodegen::render_descriptor(&literal).unwrap(),
            ("true".into(), None)
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
            TsCodegen::render_descriptor(&alias).unwrap(),
            ("Hello".into(), Some("export type Hello = true;".into()))
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
            TsCodegen::render_descriptor(&union).unwrap(),
            (
                "Hello | World".into(),
                Some(
                    r#"export type Hello = true;

export type World = "world";"#
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
            TsCodegen::render_alias(&alias).unwrap(),
            r#"export type Hi = Hello | World;

export type Hello = true;

export type World = "world";"#
        );
    }
}
