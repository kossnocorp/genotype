use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSPrimitive {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, _context: &mut Self::RenderContext) -> Result<String> {
        Ok(match self {
            TSPrimitive::String => "string",
            TSPrimitive::Number => "number",
            TSPrimitive::Boolean => "boolean",
            TSPrimitive::BigInt => "bigint",
            TSPrimitive::Null => "null",
            TSPrimitive::Undefined => "undefined",
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
            TSPrimitive::String.render(&mut Default::default()).unwrap(),
            "string"
        );
        assert_eq!(
            TSPrimitive::Number.render(&mut Default::default()).unwrap(),
            "number"
        );
        assert_eq!(
            TSPrimitive::BigInt.render(&mut Default::default()).unwrap(),
            "bigint"
        );
        assert_eq!(
            TSPrimitive::Boolean
                .render(&mut Default::default())
                .unwrap(),
            "boolean"
        );
        assert_eq!(
            TSPrimitive::Null.render(&mut Default::default()).unwrap(),
            "null"
        );
        assert_eq!(
            TSPrimitive::Undefined
                .render(&mut Default::default())
                .unwrap(),
            "undefined"
        );
    }
}
