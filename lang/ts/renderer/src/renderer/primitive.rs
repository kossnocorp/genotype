use genotype_lang_ts_parser::tree::primitive::TSPrimitive;

pub fn render_primitive<'a>(primitive: &TSPrimitive) -> String
where
    'a: 'static,
{
    match primitive {
        TSPrimitive::String => "string",
        TSPrimitive::Number => "number",
        TSPrimitive::Boolean => "boolean",
        TSPrimitive::Null => "null",
        TSPrimitive::Undefined => "undefined",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_primitive() {
        assert_eq!(render_primitive(&TSPrimitive::String), "string");
        assert_eq!(render_primitive(&TSPrimitive::Number), "number");
        assert_eq!(render_primitive(&TSPrimitive::Boolean), "boolean");
        assert_eq!(render_primitive(&TSPrimitive::Null), "null");
        assert_eq!(render_primitive(&TSPrimitive::Undefined), "undefined");
    }
}
