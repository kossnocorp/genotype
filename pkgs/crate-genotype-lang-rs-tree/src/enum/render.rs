use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSEnum {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(state, context)?);
        }

        for attribute in &self.attributes {
            blocks.push(attribute.render(state, context)?);
        }

        let name = self.name.render(state, context)?;
        blocks.push(state.indent_format(&format!("pub enum {name} {{")));

        for variant in &self.variants {
            blocks.push(variant.render(state.indent_inc(), context)?);
        }

        blocks.push(state.indent_format("}"));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::prelude::*;

    #[test]
    fn test_render() {
        assert_snapshot!(
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: Some(RSDescriptor::Primitive(RSPrimitive::String).into()),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: Some(RSDescriptor::Primitive(RSPrimitive::IntSize).into()),
                    },
                ],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        pub enum Union {
            String(String),
            Int(isize),
        }
        "
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: Some(RSDescriptor::Primitive(RSPrimitive::String).into()),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: Some(RSDescriptor::Primitive(RSPrimitive::IntSize).into()),
                    },
                ],
            }
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        pub enum Union {
            String(String),
            Int(isize),
        }
        "
        );
    }

    #[test]
    fn test_render_attributes() {
        assert_snapshot!(
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![RSAttribute("derive(Deserialize, Serialize)".into())],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: Some(RSDescriptor::Primitive(RSPrimitive::String).into()),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: Some(RSDescriptor::Primitive(RSPrimitive::IntSize).into()),
                    },
                ],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        #[derive(Deserialize, Serialize)]
        pub enum Union {
            String(String),
            Int(isize),
        }
        "
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: Some(RSDescriptor::Primitive(RSPrimitive::String).into()),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: Some(RSDescriptor::Primitive(RSPrimitive::IntSize).into()),
                    },
                ],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        /// Hello, world!
        pub enum Union {
            String(String),
            Int(isize),
        }
        "
        );
    }

    #[test]
    fn test_render_mixed() {
        assert_snapshot!(
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute("derive(Deserialize, Serialize)".into())],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: Some(RSDescriptor::Primitive(RSPrimitive::String).into()),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: Some(RSDescriptor::Primitive(RSPrimitive::IntSize).into()),
                    },
                ],
            }
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        /// Hello, world!
        #[derive(Deserialize, Serialize)]
        pub enum Union {
            String(String),
            Int(isize),
        }
        "
        );
    }

    #[test]
    fn test_render_no_descriptor() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("AnimalKind".into()));

        let union = parse_get_named::<GTUnion>(
            "Misc",
            r#"
            Misc: "hello" | 123 | true | null
            "#,
        );
        let union = union.convert(&mut context).unwrap();

        assert_snapshot!(
            union
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @r#"
        #[derive(Debug, Clone, PartialEq, Literals)]
        #[serde(untagged)]
        pub enum AnimalKind {
            #[literal("hello")]
            Hello,
            #[literal(123)]
            Lit123,
            #[literal(true)]
            True,
            #[literal(null)]
            Null,
        }
        "#
        );
    }

    #[test]
    fn test_render_literals() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("AnimalKind".into()));

        let union = Gt::union(descriptor_nodes!(
            Gt::literal_string("hello"),
            Gt::literal_boolean(true),
            Gt::primitive_boolean()
        ));
        let union = union.convert(&mut context).unwrap();

        assert_snapshot!(
            union
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @r#"
        #[derive(Debug, Clone, PartialEq, Literals)]
        #[serde(untagged)]
        pub enum AnimalKind {
            #[literal("hello")]
            Hello,
            #[literal(true)]
            True,
            Boolean(bool),
        }
        "#
        );
    }
}
