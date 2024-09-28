use crate::{name::TSName, property::TSProperty};
use genotype_lang_core::{indent::Indent, node::Node};

pub struct TSInterface {
    pub name: TSName,
    pub properties: Vec<TSProperty>,
}

impl Node for TSInterface {
    fn render(&self, indent: &Indent) -> String {
        format!("interface {} {}", self.name.render(indent), "{\n}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{indent::ts_indent, name::TSName};

    #[test]
    fn test_render_empty() {
        let indent = ts_indent();
        assert_eq!(
            TSInterface {
                name: TSName("Name".to_string()),
                properties: vec![]
            }
            .render(&indent),
            "interface Name {\n}"
        );
    }
}
