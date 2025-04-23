use genotype_lang_core_codegen::*;
use genotype_lang_core_tree::*;
use genotype_lang_rs_converter::*;
use genotype_lang_rs_tree::*;
use genotype_parser::*;
use miette::Result;

pub struct RsCodegen {}

impl RsCodegen {
    fn gen_hoisted(context: &mut RSConvertContext) -> Result<String> {
        let hoisted = context.drain_hoisted();

        Ok(RSModule::join_definitions(
            &hoisted
                .iter()
                .map(|definition| definition.render(Default::default(), &mut Default::default()))
                .collect::<Result<_>>()?,
        ))
    }
}

impl<'a, RenderState, RenderContext> GtlCodegen<'a, RenderState, RenderContext> for RsCodegen
where
    Box<(dyn GtlRenderResolveImport<'a, RenderState, RenderContext>)>: Clone,
{
    fn gen_descriptor(
        descriptor: &GTDescriptor,
    ) -> Result<GtlCodegenResultDescriptor<'a, RenderState, RenderContext>> {
        let module_id = GTModuleId("module".into());
        let mut context = RSConvertContext::empty(module_id);
        let converted = descriptor.convert(&mut context)?;

        let inline = converted.render(Default::default(), &mut Default::default())?;
        let definitions = Self::gen_hoisted(&mut context)?;

        Ok(GtlCodegenResultDescriptor {
            inline,
            definitions,
            // [TODO]
            resolve: GtlRenderResolve {
                imports: vec![],
                exports: vec![],
            },
        })
    }

    fn gen_alias(alias: &GTAlias) -> Result<GtlCodegenResultAlias<'a, RenderState, RenderContext>> {
        let mut definitions = vec![];

        let module_id = GTModuleId("module".into());
        let mut context = RSConvertContext::empty(module_id);
        let converted = alias.convert(&mut context)?;

        let rendered_alias = converted.render(Default::default(), &mut Default::default())?;
        definitions.push(rendered_alias);

        let rendered_hoisted = Self::gen_hoisted(&mut context)?;
        if !rendered_hoisted.is_empty() {
            definitions.push(rendered_hoisted);
        }

        let definitions = RSModule::join_definitions(&definitions);

        Ok(GtlCodegenResultAlias {
            definitions,
            // [TODO]
            resolve: GtlRenderResolve {
                imports: vec![],
                exports: vec![],
            },
        })
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
            RsCodegen::gen_descriptor(&literal).unwrap(),
            GtlCodegenResultDescriptor {
                inline: "True".into(),
                definitions: r#"#[literal(true)]
pub struct True;"#
                    .into(),
                resolve: GtlRenderResolve::<RSRenderState, RSRenderContext> {
                    imports: vec![],
                    exports: vec![],
                },
            }
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
            RsCodegen::gen_descriptor(&alias).unwrap(),
            GtlCodegenResultDescriptor {
                inline: "Hello".into(),
                definitions: r#"#[literal(true)]
pub struct HelloTrue;

pub type Hello = HelloTrue;"#
                    .into(),
                resolve: GtlRenderResolve::<RSRenderState, RSRenderContext> {
                    imports: vec![],
                    exports: vec![],
                },
            }
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
            RsCodegen::gen_descriptor(&union).unwrap(),
            GtlCodegenResultDescriptor {
                inline: "".into(),
                definitions: r#"#[literal(true)]
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
                .into(),
                resolve: GtlRenderResolve::<RSRenderState, RSRenderContext> {
                    imports: vec![],
                    exports: vec![],
                },
            }
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
            RsCodegen::gen_alias(&alias).unwrap(),
            GtlCodegenResultAlias {
                definitions: r#"#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
                    .into(),
                resolve: GtlRenderResolve::<RSRenderState, RSRenderContext> {
                    imports: vec![],
                    exports: vec![],
                },
            }
        );
    }
}
