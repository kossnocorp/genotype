use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYClass {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;
        let extensions = self.render_extensions(state, context)?;
        let body = self.render_body(state, context)?;

        Ok(state.indent_format(&format!("class {name}{extensions}:\n{body}")))
    }
}

impl<'a> PYClass {
    fn render_extensions(
        &self,
        state: PYRenderState,
        context: &mut PYRenderContext<'a>,
    ) -> Result<String> {
        let mut extensions = self
            .extensions
            .iter()
            .map(|extension| extension.render(state, context))
            .collect::<Result<Vec<_>>>()?;
        // [TODO] Push model when converting instead
        extensions.push("Model".into());

        let extensions = extensions.join(", ");

        Ok(if extensions.len() > 0 {
            format!("({extensions})")
        } else {
            "".into()
        })
    }

    fn render_body(
        &self,
        state: PYRenderState,
        context: &mut PYRenderContext<'a>,
    ) -> Result<String> {
        let mut body = vec![];

        if let Some(doc) = &self.doc {
            body.push(doc.render(state.indent_inc(), context)?);
        }

        if self.properties.len() > 0 {
            body.push(self.render_properties(state, context)?);
        } else {
            body.push(state.indent_inc().indent_format("pass"));
        }

        Ok(body.join("\n\n"))
    }

    fn render_properties(
        &self,
        state: PYRenderState,
        context: &mut PYRenderContext<'a>,
    ) -> Result<String> {
        Ok(self
            .properties
            .iter()
            .map(|property| property.render(state.indent_inc(), context))
            .collect::<Result<Vec<_>>>()?
            .join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"class Name(Model):
    pass"#
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true
                    },
                    PYProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                        required: false
                    }
                ],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"class Name(Model):
    name: str
    age: Optional[int] = None"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true
                    },
                    PYProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                        required: false
                    }
                ],
                references: vec![],
            }
            .render(
                PYRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            r#"    class Name(Model):
        name: str
        age: Optional[int] = None"#
        );
    }

    #[test]
    fn test_render_extensions() {
        assert_eq!(
            PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![
                    PYReference::new("Hello".into(), false).into(),
                    PYReference::new("World".into(), false).into()
                ],
                properties: vec![PYProperty {
                    doc: None,
                    name: "name".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    required: true
                }],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"class Name(Hello, World, Model):
    name: str"#
        );
    }

    #[test]
    fn test_render_doc_empty() {
        assert_eq!(
            PYClass {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"class Name(Model):
    """Hello, world!"""

    pass"#
        );
    }

    #[test]
    fn test_render_doc_properties() {
        assert_eq!(
            PYClass {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![PYProperty {
                    doc: None,
                    name: "name".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    required: true
                }],
                references: vec![],
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"class Name(Model):
    """Hello, world!"""

    name: str"#
        );
    }
}
