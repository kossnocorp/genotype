use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSRecord;

impl GTRender for TSRecord {
    fn render(&self, indent: &GTIndent) -> String {
        format!(
            "Record<{}, {}>",
            self.key.render(indent),
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
            TSRecord {
                key: TSRecordKey::Number,
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(&ts_indent()),
            "Record<number, string>"
        );
    }
}
