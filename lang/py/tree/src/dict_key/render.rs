use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYDictKey {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, _context: &mut Self::RenderContext) -> Result<String> {
        Ok(match self {
            PYDictKey::String => "str".into(),
            PYDictKey::Int => "int".into(),
            PYDictKey::Float => "float".into(),
            PYDictKey::Boolean => "bool".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_eq!(
            PYDictKey::Boolean.render(&mut Default::default()).unwrap(),
            "bool"
        );
        assert_eq!(
            PYDictKey::String.render(&mut Default::default()).unwrap(),
            "str"
        );
        assert_eq!(
            PYDictKey::Int.render(&mut Default::default()).unwrap(),
            "int"
        );
        assert_eq!(
            PYDictKey::Float.render(&mut Default::default()).unwrap(),
            "float"
        );
    }
}
