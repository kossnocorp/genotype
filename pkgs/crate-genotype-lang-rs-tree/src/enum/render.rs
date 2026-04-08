use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsEnum {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

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

    #[test]
    fn test_render() {
        assert_snapshot!(
            RsEnum {
                id: GtDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![],
                name: "Union".into(),
                variants: vec![
                    RsEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: Some(RsDescriptor::Primitive(RsPrimitive::String).into()),
                    },
                    RsEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: Some(RsDescriptor::Primitive(RsPrimitive::IntSize).into()),
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
            RsEnum {
                id: GtDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![],
                name: "Union".into(),
                variants: vec![
                    RsEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: Some(RsDescriptor::Primitive(RsPrimitive::String).into()),
                    },
                    RsEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: Some(RsDescriptor::Primitive(RsPrimitive::IntSize).into()),
                    },
                ],
            }
            .render(
                RsRenderState::default().indent_inc(),
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
            RsEnum {
                id: GtDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![RsAttribute("derive(Deserialize, Serialize)".into())],
                name: "Union".into(),
                variants: vec![
                    RsEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: Some(RsDescriptor::Primitive(RsPrimitive::String).into()),
                    },
                    RsEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: Some(RsDescriptor::Primitive(RsPrimitive::IntSize).into()),
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
            RsEnum {
                id: GtDefinitionId("module".into(), "Union".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![],
                name: "Union".into(),
                variants: vec![
                    RsEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: Some(RsDescriptor::Primitive(RsPrimitive::String).into()),
                    },
                    RsEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: Some(RsDescriptor::Primitive(RsPrimitive::IntSize).into()),
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
            RsEnum {
                id: GtDefinitionId("module".into(), "Union".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![RsAttribute("derive(Deserialize, Serialize)".into())],
                name: "Union".into(),
                variants: vec![
                    RsEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: Some(RsDescriptor::Primitive(RsPrimitive::String).into()),
                    },
                    RsEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: Some(RsDescriptor::Primitive(RsPrimitive::IntSize).into()),
                    },
                ],
            }
            .render(
                RsRenderState::default().indent_inc(),
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
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("AnimalKind".into()));

        let union = parse_get_named::<GtUnion>(
            "Misc",
            r#"
            Misc: "hello" | 123 | true | null
            "#,
        );
        let union = union.convert(&mut context).unwrap();

        assert_snapshot!(
            union
            .render(
                RsRenderState::default().indent_inc(),
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
        let mut context = RsConvertContext::empty("module".into());
        context.enter_parent(RsContextParent::Alias("AnimalKind".into()));

        let union = Gt::union(vec_into!(
            Gt::literal_string("hello"),
            Gt::literal_boolean(true),
            Gt::primitive_boolean()
        ));
        let union = union.convert(&mut context).unwrap();

        assert_snapshot!(
            union
            .render(
                RsRenderState::default().indent_inc(),
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
