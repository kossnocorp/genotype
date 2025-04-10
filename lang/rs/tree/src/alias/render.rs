use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSAlias {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let name = self.name.render(context)?;
        let descriptor = self.descriptor.render(context)?;
        let r#type = format!("pub type {name} = {descriptor};");

        Ok(if let Some(doc) = &self.doc {
            format!("{}\n{}", doc.render(context)?, r#type)
        } else {
            r#type
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&mut Default::default())
            .unwrap(),
            "pub type Name = String;"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: Some("Hello, world!".into()),
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"/// Hello, world!
pub type Name = String;"#
        );
    }
}
