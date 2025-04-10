use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYNewtype {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut PYRenderContext) -> Result<String> {
        let mut blocks = vec![];

        let name = self.name.render(context)?;
        let primitive = self.primitive.render(context)?;
        blocks.push(format!(r#"{name} = NewType("{name}", {primitive})"#));

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(context)?);
        }

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            PYNewtype {
                doc: None,
                name: "UserId".into(),
                primitive: PYPrimitive::String,
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"UserId = NewType("UserId", str)"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            PYNewtype {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "UserId".into(),
                primitive: PYPrimitive::String,
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"UserId = NewType("UserId", str)
"""Hello, world!""""#
        );
    }
}
