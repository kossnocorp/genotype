use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYClass {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let name = self.name.render(context)?;
        let extensions = self.render_extensions(context)?;
        let body = self.render_body(context)?;

        Ok(context.indent_format(&format!("class {name}{extensions}:\n{body}")))
    }
}

impl<'a> PYClass {
    fn render_extensions(&self, context: &mut PYRenderContext<'a>) -> Result<String> {
        let mut extensions = self
            .extensions
            .iter()
            .map(|extension| extension.render(context))
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

    fn render_body(&self, context: &mut PYRenderContext<'a>) -> Result<String> {
        let mut body = vec![];

        if let Some(doc) = &self.doc {
            body.push(doc.render(&mut context.indent_inc())?);
        }

        if self.properties.len() > 0 {
            body.push(self.render_properties(context)?);
        } else {
            body.push(context.indent_inc().indent_format("pass"));
        }

        Ok(body.join("\n\n"))
    }

    fn render_properties(&self, context: &mut PYRenderContext<'a>) -> Result<String> {
        let mut property_context = context.indent_inc();
        Ok(self
            .properties
            .iter()
            .map(|property| property.render(&mut property_context))
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
            .render(&mut Default::default())
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
            .render(&mut Default::default())
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
            .render(&mut PYRenderContext::default().indent_inc())
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
            .render(&mut Default::default())
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
            .render(&mut Default::default())
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
            .render(&mut Default::default())
            .unwrap(),
            r#"class Name(Model):
    """Hello, world!"""

    name: str"#
        );
    }
}
