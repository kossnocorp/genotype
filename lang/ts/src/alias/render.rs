use genotype_lang_core::{indent::GTIndent, render::GTRender};

use super::TSAlias;

impl GTRender for TSAlias {
    fn render(&self, indent: &GTIndent) -> String {
        format!(
            "type {} = {};",
            self.name.render(indent),
            self.descriptor.render(indent)
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        indent::ts_indent, name::TSName, primitive::TSPrimitive, type_descriptor::TSTypeDescriptor,
    };

    #[test]
    fn test_render() {
        let indent = ts_indent();
        assert_eq!(
            TSAlias {
                name: TSName("Name".to_string()),
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String)
            }
            .render(&indent),
            "type Name = string;"
        );
        assert_eq!(
            TSTypeDescriptor::Primitive(TSPrimitive::String).render(&indent),
            "string"
        );
    }
}
