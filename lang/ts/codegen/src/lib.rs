use genotype_lang_core_codegen::*;
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
    fn gen_hoisted(context: &mut TSConvertContext) -> String {
        let hoisted = context.drain_hoisted();

        TSModule::join_definitions(
            hoisted
                .iter()
                .map(|definition| definition.render(&ts_indent()))
                .collect(),
        )
    }
}

impl GtlCodegen for TsCodegen {
    fn indent() -> GTIndent<'static> {
        ts_indent()
    }

    fn gen_descriptor(descriptor: &GTDescriptor) -> Result<GtlCodegenResultDescriptor> {
        let mut context = TSConvertContext::default();
        let converted = descriptor.convert(&mut context);

        let inline = converted.render(&ts_indent());
        let definitions = Self::gen_hoisted(&mut context);

        Ok(GtlCodegenResultDescriptor {
            inline,
            definitions,
            // [TODO]
            resolve: GtlCodegenResolve {
                imports: vec![],
                claims: vec![],
            },
        })
    }

    fn gen_alias(alias: &GTAlias) -> Result<GtlCodegenResultAlias> {
        let mut definitions = vec![];

        let mut context = TSConvertContext::default();
        let converted = alias.convert(&mut context);

        let rendered_alias = converted.render(&ts_indent());
        definitions.push(rendered_alias);

        let rendered_hoisted = Self::gen_hoisted(&mut context);
        if !rendered_hoisted.is_empty() {
            definitions.push(rendered_hoisted);
        }

        let definitions = TSModule::join_definitions(definitions);

        Ok(GtlCodegenResultAlias {
            definitions,
            // [TODO]
            resolve: GtlCodegenResolve {
                imports: vec![],
                claims: vec![],
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_lang_core_codegen::GtlCodegenResolve;
    use genotype_parser::{GTAlias, GTDefinitionId, GTIdentifier, GTLiteral, GTModuleId, GTUnion};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_descriptor() {
        let literal = GTDescriptor::Literal(GTLiteral::Boolean(Default::default(), true));
        assert_eq!(
            TsCodegen::gen_descriptor(&literal).unwrap(),
            GtlCodegenResultDescriptor {
                inline: "true".into(),
                definitions: "".into(),
                resolve: GtlCodegenResolve {
                    imports: vec![],
                    claims: vec![],
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
            TsCodegen::gen_descriptor(&alias).unwrap(),
            GtlCodegenResultDescriptor {
                inline: "Hello".into(),
                definitions: "export type Hello = true;".into(),
                resolve: GtlCodegenResolve {
                    imports: vec![],
                    claims: vec![],
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
            TsCodegen::gen_descriptor(&union).unwrap(),
            GtlCodegenResultDescriptor {
                inline: "Hello | World".into(),
                definitions: r#"export type Hello = true;

export type World = "world";"#
                    .into(),
                resolve: GtlCodegenResolve {
                    imports: vec![],
                    claims: vec![],
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
            TsCodegen::gen_alias(&alias).unwrap(),
            GtlCodegenResultAlias {
                definitions: r#"export type Hi = Hello | World;

export type Hello = true;

export type World = "world";"#
                    .into(),
                resolve: GtlCodegenResolve {
                    imports: vec![],
                    claims: vec![],
                },
            }
        );
    }
}
