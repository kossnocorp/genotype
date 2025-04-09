use genotype_lang_core_codegen::GtlCodegen;
use genotype_lang_core_tree::{indent::GTIndent, render::GTRenderModule};
use genotype_lang_rs_converter::{context::RSConvertContext, convert::RSConvert};
use genotype_lang_rs_tree::*;
use genotype_parser::*;
use miette::Result;

pub struct RsCodegen {}

impl RsCodegen {
    fn render_hoisted(context: &mut RSConvertContext) -> Result<Option<String>> {
        let hoisted = context.drain_hoisted();
        Ok(if hoisted.is_empty() {
            None
        } else {
            Some(RSModule::join_definitions(
                hoisted
                    .iter()
                    .map(|definition| definition.render(&rs_indent(), &Default::default()))
                    .collect::<Result<_>>()?,
            ))
        })
    }
}

impl GtlCodegen for RsCodegen {
    fn indent() -> GTIndent<'static> {
        rs_indent()
    }

    fn render_descriptor(descriptor: &GTDescriptor) -> Result<(String, Option<String>)> {
        let module_id = GTModuleId("module".into());
        let mut context = RSConvertContext::empty(module_id);
        let converted = descriptor.convert(&mut context)?;
        let rendered = converted.render(&rs_indent(), &Default::default())?;
        let hoisted_rendered = Self::render_hoisted(&mut context)?;
        Ok((rendered, hoisted_rendered))
    }

    fn render_alias(alias: &GTAlias) -> Result<String> {
        let mut rendered = vec![];

        let module_id = GTModuleId("module".into());
        let mut context = RSConvertContext::empty(module_id);
        let converted = alias.convert(&mut context)?;

        let rendered_alias = converted.render(&rs_indent(), &Default::default())?;
        rendered.push(rendered_alias);

        if let Some(hoisted_rendered) = Self::render_hoisted(&mut context)? {
            rendered.push(hoisted_rendered);
        }

        Ok(RSModule::join_definitions(rendered))
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
            RsCodegen::render_descriptor(&literal).unwrap(),
            (
                "True".into(),
                Some(
                    r#"#[literal(true)]
pub struct True;"#
                        .into()
                )
            )
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
            RsCodegen::render_descriptor(&alias).unwrap(),
            (
                "Hello".into(),
                Some(
                    r#"#[literal(true)]
pub struct HelloTrue;

pub type Hello = HelloTrue;"#
                        .into()
                )
            )
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
            RsCodegen::render_descriptor(&union).unwrap(),
            (
                "".into(),
                Some(
                    r#"#[literal(true)]
pub struct HelloTrue;

pub type Hello = HelloTrue;

#[literal("world")]
pub struct WorldWorld;

pub type World = WorldWorld;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum  {
    Hello(Hello),
    World(World),
}"#
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
            RsCodegen::render_alias(&alias).unwrap(),
            r#"#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Hi {
    Hello(Hello),
    World(World),
}

#[literal(true)]
pub struct HiHelloTrue;

pub type Hello = HiHelloTrue;

#[literal("world")]
pub struct HiWorldWorld;

pub type World = HiWorldWorld;"#
        );
    }
}
