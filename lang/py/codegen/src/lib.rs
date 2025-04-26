use genotype_lang_core_codegen::*;
use genotype_lang_core_tree::*;
use genotype_lang_py_tree::*;
use genotype_parser::*;
use miette::Result;

pub struct PyCodegen {}

impl PyCodegen {
    fn gen_hoisted(context: &mut PYConvertContext) -> Result<String> {
        let hoisted = context.drain_hoisted();

        Ok(PYModule::join_definitions(
            &hoisted
                .iter()
                .map(|definition| definition.render(Default::default(), &mut Default::default()))
                .collect::<Result<_>>()?,
        ))
    }
}

impl<'a, RenderState, RenderContext> GtlCodegen<'a, RenderState, RenderContext> for PyCodegen
where
    Box<(dyn GtlRenderResolveImport<'a, RenderState, RenderContext>)>: Clone,
{
    fn gen_descriptor(
        descriptor: &GTDescriptor,
    ) -> Result<GtlCodegenResultDescriptor<'a, RenderState, RenderContext>> {
        let mut context = PYConvertContext::default();
        let converted = descriptor.convert(&mut context);

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

        let mut context = PYConvertContext::default();
        let converted = genotype_parser::GTAlias::convert(alias, &mut context);

        let rendered_alias = converted.render(Default::default(), &mut Default::default())?;
        definitions.push(rendered_alias);

        let rendered_hoisted = Self::gen_hoisted(&mut context)?;
        if !rendered_hoisted.is_empty() {
            definitions.push(rendered_hoisted);
        }

        let definitions = PYModule::join_definitions(&definitions);

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
mod tespy {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_descriptor() {
        let literal = GTDescriptor::Literal(GTLiteral::Boolean(Default::default(), true));
        assert_eq!(
            PyCodegen::gen_descriptor(&literal).unwrap(),
            GtlCodegenResultDescriptor {
                inline: "Literal[True]".into(),
                definitions: "".into(),
                resolve: GtlRenderResolve::<PYRenderState, PYRenderContext> {
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
            PyCodegen::gen_descriptor(&alias).unwrap(),
            GtlCodegenResultDescriptor {
                inline: "Hello".into(),
                definitions: "type Hello = Literal[True]".into(),
                resolve: GtlRenderResolve::<PYRenderState, PYRenderContext> {
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
            PyCodegen::gen_descriptor(&union).unwrap(),
            GtlCodegenResultDescriptor {
                inline: "Hello | World".into(),
                definitions: r#"type Hello = Literal[True]


type World = Literal["world"]"#
                    .into(),
                resolve: GtlRenderResolve::<PYRenderState, PYRenderContext> {
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
        assert_eq!(PyCodegen::gen_alias(&alias).unwrap(), {
            GtlCodegenResultAlias {
                definitions: r#"type Hi = Hello | World


type Hello = Literal[True]


type World = Literal["world"]"#
                    .into(),
                resolve: GtlRenderResolve::<PYRenderState, PYRenderContext> {
                    imports: vec![],
                    exports: vec![],
                },
            }
        });
    }
}
