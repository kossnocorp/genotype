use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYPrimitive {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, _context: &mut PYRenderContext) -> Result<String> {
        Ok(match self {
            PYPrimitive::Boolean => "bool",
            PYPrimitive::String => "str",
            PYPrimitive::Int => "int",
            PYPrimitive::Float => "float",
            PYPrimitive::None => "None",
        }
        .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            PYPrimitive::Boolean
                .render(&mut Default::default())
                .unwrap(),
            "bool"
        );
        assert_eq!(
            PYPrimitive::String.render(&mut Default::default()).unwrap(),
            "str"
        );
        assert_eq!(
            PYPrimitive::Int.render(&mut Default::default()).unwrap(),
            "int"
        );
        assert_eq!(
            PYPrimitive::Float.render(&mut Default::default()).unwrap(),
            "float"
        );
        assert_eq!(
            PYPrimitive::None.render(&mut Default::default()).unwrap(),
            "None"
        );
    }
}
