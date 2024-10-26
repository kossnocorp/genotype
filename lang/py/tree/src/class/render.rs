use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_py_config::PYLangConfig;

use crate::PYRender;

use super::PYClass;

impl PYRender for PYClass {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        let name = self.name.render(indent);
        let extensions = self.render_extensions(indent, config);
        let body = self.render_body(indent, config);

        format!("{}class {name}{extensions}:\n{body}", indent.string)
    }
}

impl PYClass {
    fn render_extensions(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        let mut extensions = self
            .extensions
            .iter()
            .map(|extension| extension.render(indent, config))
            .collect::<Vec<_>>();
        extensions.push("Model".into());

        let extensions = extensions.join(", ");

        if extensions.len() > 0 {
            format!("({extensions})")
        } else {
            "".into()
        }
    }

    fn render_body(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        let mut body = vec![];

        if let Some(doc) = &self.doc {
            body.push(doc.render(&indent.increment()));
        }

        if self.properties.len() > 0 {
            body.push(self.render_properties(indent, config));
        } else {
            body.push(indent.increment().format("pass"));
        }

        body.join("\n\n")
    }

    fn render_properties(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        let indent = indent.increment();
        self.properties
            .iter()
            .map(|property| property.render(&indent, config))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(&py_indent(), &Default::default()),
            r#"class Name(Model):
    pass"#
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true
                    },
                    PYProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                        required: false
                    }
                ]
            }
            .render(&py_indent(), &Default::default()),
            r#"class Name(Model):
    name: str
    age: Optional[int] = None"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true
                    },
                    PYProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                        required: false
                    }
                ]
            }
            .render(&py_indent().increment(), &Default::default()),
            r#"    class Name(Model):
        name: str
        age: Optional[int] = None"#
        );
    }

    #[test]
    fn test_render_extensions() {
        assert_eq!(
            PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![
                    PYReference::new("Hello".into(), false).into(),
                    PYReference::new("World".into(), false).into()
                ],
                properties: vec![PYProperty {
                    doc: None,
                    name: "name".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    required: true
                },]
            }
            .render(&py_indent(), &Default::default()),
            r#"class Name(Hello, World, Model):
    name: str"#
        );
    }

    #[test]
    fn test_render_doc_empty() {
        assert_eq!(
            PYClass {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(&py_indent(), &Default::default()),
            r#"class Name(Model):
    """Hello, world!"""

    pass"#
        );
    }

    #[test]
    fn test_render_doc_properties() {
        assert_eq!(
            PYClass {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![PYProperty {
                    doc: None,
                    name: "name".into(),
                    descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                    required: true
                },]
            }
            .render(&py_indent(), &Default::default()),
            r#"class Name(Model):
    """Hello, world!"""

    name: str"#
        );
    }
}
