use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSIntersection {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        Ok(self
            .descriptors
            .iter()
            .map(|d| d.render(context))
            .collect::<Result<Vec<_>>>()?
            .join(" & "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_union() {
        assert_eq!(
            TSIntersection {
                descriptors: vec![
                    TSObject {
                        properties: vec![TSProperty {
                            doc: None,
                            name: "hello".into(),
                            descriptor: TSPrimitive::String.into(),
                            required: true,
                        }],
                    }
                    .into(),
                    "World".into(),
                ]
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"{
  hello: string
} & World"#
        );
    }
}
