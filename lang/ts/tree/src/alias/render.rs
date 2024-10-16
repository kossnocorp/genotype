use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

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
    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            TSAlias {
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(&ts_indent()),
            "type Name = string;"
        );
    }
}
