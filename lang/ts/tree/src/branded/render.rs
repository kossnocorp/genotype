use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use heck::ToLowerCamelCase;

use super::TSBranded;

impl GTRender for TSBranded {
    fn render(&self, indent: &GTIndent) -> String {
        let brand_name = format!("{brand}Brand", brand = self.name.0).to_lower_camel_case();

        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(indent));
        }

        blocks.push(format!(
            "{indent}export type {name} = {primitive} & {{ [{brand_name}]: true }};",
            indent = indent.string,
            name = self.name.render(indent),
            primitive = self.primitive.render(indent)
        ));

        blocks.push(format!(
            "{indent}declare const {brand_name}: unique symbol;",
            indent = indent.string,
        ));

        blocks.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{indent::ts_indent, TSDoc, TSPrimitive};

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            TSBranded {
                doc: None,
                name: "Version".into(),
                primitive: TSPrimitive::Number
            }
            .render(&ts_indent()),
            r#"export type Version = number & { [versionBrand]: true };
declare const versionBrand: unique symbol;"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            TSBranded {
                doc: Some(TSDoc("Object version.".into())),
                name: "Version".into(),
                primitive: TSPrimitive::Number
            }
            .render(&ts_indent()),
            r#"/** Object version. */
export type Version = number & { [versionBrand]: true };
declare const versionBrand: unique symbol;"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            TSBranded {
                doc: Some(TSDoc("Object version.".into())),
                name: "Version".into(),
                primitive: TSPrimitive::Number
            }
            .render(&ts_indent().increment()),
            r#"  /** Object version. */
  export type Version = number & { [versionBrand]: true };
  declare const versionBrand: unique symbol;"#
        );
    }
}
