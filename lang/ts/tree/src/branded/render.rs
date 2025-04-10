use crate::*;
use genotype_lang_core_tree::*;
use heck::ToLowerCamelCase;
use miette::Result;

impl<'a> GtlRender<'a> for TSBranded {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let brand_name = format!("{brand}Brand", brand = self.name.0).to_lower_camel_case();

        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(context)?);
        }

        blocks.push(format!(
            "{indent}export type {name} = {primitive} & {{ [{brand_name}]: true }};",
            indent = context.indent_legacy.string.clone(),
            name = self.name.render(context)?,
            primitive = self.primitive.render(context)?
        ));

        blocks.push(format!(
            "{indent}declare const {brand_name}: unique symbol;",
            indent = context.indent_legacy.string,
        ));

        Ok(blocks.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            TSBranded {
                doc: None,
                name: "Version".into(),
                primitive: TSPrimitive::Number
            }
            .render(&mut Default::default())
            .unwrap(),
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
            .render(&mut Default::default())
            .unwrap(),
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
            .render(&mut TSRenderContext::default().indent_inc())
            .unwrap(),
            r#"  /** Object version. */
  export type Version = number & { [versionBrand]: true };
  declare const versionBrand: unique symbol;"#
        );
    }
}
