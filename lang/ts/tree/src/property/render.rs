use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSProperty {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let name = self.name.render(context)?;
        let descriptor = self.descriptor.render(context)?;
        let str = format!(
            "{}{name}{}: {descriptor}",
            context.indent_legacy.string.clone(),
            if self.required { "" } else { "?" },
        );

        TSDoc::with_doc(&self.doc, context, str, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            TSProperty {
                doc: None,
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: true
            }
            .render(&mut Default::default())
            .unwrap(),
            "name: string"
        );
        assert_eq!(
            TSProperty {
                doc: None,
                name: "name".into(),
                descriptor: TSDescriptor::Reference("Name".into()),
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
            TSProperty {
                doc: None,
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: true
            }
            .render(&mut TSRenderContext::default().indent_inc())
            .unwrap(),
            "  name: string"
        );
    }

    #[test]
    fn test_render_required() {
        assert_eq!(
            TSProperty {
                doc: None,
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: false
            }
            .render(&mut Default::default())
            .unwrap(),
            "name?: string"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            TSProperty {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: true
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"/** Hello, world! */
name: string"#
        );
    }
}
