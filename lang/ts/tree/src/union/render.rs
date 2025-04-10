use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSUnion {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        Ok(self
            .descriptors
            .iter()
            .map(|d| d.render(context))
            .collect::<Result<Vec<_>>>()?
            .join(" | "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_union() {
        assert_eq!(
            TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::String),
                    TSDescriptor::Primitive(TSPrimitive::Number),
                ]
            }
            .render(&mut Default::default())
            .unwrap(),
            "string | number"
        );
    }
}
