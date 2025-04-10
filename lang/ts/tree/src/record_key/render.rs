use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSRecordKey {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, _context: &mut Self::RenderContext) -> Result<String> {
        Ok(match self {
            TSRecordKey::String => "string".into(),
            TSRecordKey::Number => "number".into(),
            TSRecordKey::Boolean => "boolean".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        assert_eq!(
            TSRecordKey::String.render(&mut Default::default()).unwrap(),
            "string"
        );
        assert_eq!(
            TSRecordKey::Number.render(&mut Default::default()).unwrap(),
            "number"
        );
        assert_eq!(
            TSRecordKey::Boolean
                .render(&mut Default::default())
                .unwrap(),
            "boolean"
        );
    }
}
