use genotype_lang_core::{indent::Indent, node::Node};

#[derive(Debug, PartialEq, Clone)]
pub struct TSName(pub String);

impl Node for TSName {
    fn render(&self, _indent: &Indent) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render_name() {
        let indent = ts_indent();
        assert_eq!(TSName("foo".to_string()).render(&indent), "foo");
    }
}
