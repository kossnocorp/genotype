use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSAlias {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let name = self.name.render(context)?;
        let descriptor = self.descriptor.render(context)?;

        TSDoc::with_doc(
            &self.doc,
            context,
            format!("export type {name} = {descriptor};",),
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            TSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(&mut Default::default())
            .unwrap(),
            "export type Name = string;"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            TSAlias {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"/** Hello, world! */
export type Name = string;"#
        );
    }
}
