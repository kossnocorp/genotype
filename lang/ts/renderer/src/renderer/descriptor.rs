use genotype_lang_ts_parser::tree::descriptor::TSDescriptor;

use super::primitive::render_primitive;

pub fn render_descriptor(descriptor: &TSDescriptor) -> String {
    match descriptor {
        TSDescriptor::Primitive(primitive) => render_primitive(primitive),
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_parser::tree::primitive::TSPrimitive;

    use super::*;

    #[test]
    fn test_primitive() {
        assert_eq!(
            render_descriptor(&TSDescriptor::Primitive(TSPrimitive::Boolean)),
            "boolean"
        );
        assert_eq!(
            render_descriptor(&TSDescriptor::Primitive(TSPrimitive::String)),
            "string"
        );
    }
}
