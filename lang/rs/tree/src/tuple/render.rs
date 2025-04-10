use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSTuple {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(context))
            .collect::<Result<Vec<String>>>()?
            .join(", ");
        Ok(format!("({descriptors})"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            RSTuple {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::String),
                    RSDescriptor::Primitive(RSPrimitive::IntSize),
                ]
            }
            .render(&mut Default::default())
            .unwrap(),
            "(String, isize)"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_eq!(
            RSTuple {
                descriptors: vec![]
            }
            .render(&mut Default::default())
            .unwrap(),
            "()"
        );
    }
}
