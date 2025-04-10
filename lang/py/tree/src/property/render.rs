use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYProperty {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let name = self.name.render(context)?;

        let descriptor = self.descriptor.render(context)?;
        let descriptor = if self.required {
            descriptor
        } else {
            format!("Optional[{descriptor}] = None")
        };

        let doc = if let Some(doc) = &self.doc {
            format!("\n{}", doc.render(context)?)
        } else {
            "".into()
        };

        Ok(context.indent_format(&format!("{name}: {descriptor}{doc}",)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            PYProperty {
                doc: None,
                name: "name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                required: true
            }
            .render(&mut Default::default())
            .unwrap(),
            "name: str"
        );
        assert_eq!(
            PYProperty {
                doc: None,
                name: "name".into(),
                descriptor: PYReference::new("Name".into(), false).into(),
                required: true
            }
            .render(&mut Default::default())
            .unwrap(),
            "name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            PYProperty {
                doc: None,
                name: "name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                required: true
            }
            .render(&mut PYRenderContext::default().indent_inc())
            .unwrap(),
            "    name: str"
        );
    }

    #[test]
    fn test_render_required() {
        assert_eq!(
            PYProperty {
                doc: None,
                name: "name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                required: false
            }
            .render(&mut Default::default())
            .unwrap(),
            "name: Optional[str] = None"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            PYProperty {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                required: false
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"name: Optional[str] = None
"""Hello, world!""""#
        );
    }
}
