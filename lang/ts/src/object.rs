use genotype_lang_core::{indent::Indent, node::Node};

pub struct TSObject {}

impl Node for TSObject {
    fn render(&self, indent: &Indent) -> String {
        format!("{}\n{}", "{", indent.format("}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::indent::ts_indent;

    use super::*;

    #[test]
    fn test_render_empty() {
        let indent = ts_indent();
        assert_eq!(TSObject {}.render(&indent), "{\n}");
        let indent = indent.increment();
        assert_eq!(TSObject {}.render(&indent), "{\n  }");
    }
}
