use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsModule {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            let doc = doc.render(state, context)?;
            if !doc.is_empty() {
                blocks.push(doc);
            }
        }

        let imports = Self::join_imports(
            &self
                .imports
                .iter()
                .map(|import| import.render(state, context))
                .collect::<Result<Vec<String>>>()?,
        );

        if !imports.is_empty() {
            blocks.push(imports);
        }

        let definitions = Self::join_definitions(
            &self
                .definitions
                .iter()
                .map(|definition| definition.render(state, context))
                .collect::<Result<Vec<String>>>()?,
        );

        if !definitions.is_empty() {
            blocks.push(definitions);
        }

        Ok(Self::join_blocks(&blocks))
    }
}

impl GtlRenderModule for RsModule {}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::GtDefinitionId;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsModule {
                id: "module".into(),
                doc: None,
                imports: vec![
                    RsUse {
                        reference: RsUseReference::Module,
                        dependency: RsDependencyIdent::Local(RsPath(
                            "path/to/module".into(),
                            "self::path::to::module".into()
                        ))
                    },
                    RsUse {
                        reference: RsUseReference::Named(vec![
                            RsUseName::Name("Name".into()),
                            RsUseName::Alias("Name".into(), "Alias".into()),
                        ]),
                        dependency: RsDependencyIdent::Local(RsPath(
                            "path/to/module".into(),
                            "self::path::to::module".into()
                        ))
                    }
                ],
                definitions: vec![
                    RsDefinition::Alias(RsAlias {
                        id: GtDefinitionId("module".into(), "Name".into()),
                        doc: None,
                        name: "Name".into(),
                        descriptor: RsDescriptor::Primitive(RsPrimitive::String),
                    }),
                    RsDefinition::Struct(RsStruct {
                        id: GtDefinitionId("module".into(), "Name".into()),
                        doc: None,
                        attributes: vec![],
                        name: "Name".into(),
                        fields: vec![
                            RsField {
                                doc: None,
                                attributes: vec![],
                                name: "name".into(),
                                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
                            },
                            RsField {
                                doc: None,
                                attributes: vec![],
                                name: "age".into(),
                                descriptor: RsDescriptor::Primitive(RsPrimitive::IntSize),
                            }
                        ]
                        .into(),
                    }),
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        use self::path::to::module;
        use self::path::to::module::{Name, Name as Alias};

        pub type Name = String;

        pub struct Name {
            pub name: String,
            pub age: isize,
        }
        "
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            RsModule {
                id: "module".into(),
                doc: Some(RsDoc::new("Hello, world!", true)),
                imports: vec![RsUse {
                    reference: RsUseReference::Module,
                    dependency: RsDependencyIdent::Local(RsPath(
                        "path/to/module".into(),
                        "self::path::to::module".into()
                    ))
                },],
                definitions: vec![RsDefinition::Alias(RsAlias {
                    id: GtDefinitionId("module".into(), "Name".into()),
                    doc: None,
                    name: "Name".into(),
                    descriptor: RsDescriptor::Primitive(RsPrimitive::String),
                })]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        //! Hello, world!

        use self::path::to::module;

        pub type Name = String;
        "
        );
    }
}
