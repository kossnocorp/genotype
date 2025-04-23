use crate::prelude::internal::*;
use heck::ToLowerCamelCase;

impl<'a> GtlRender<'a> for TSBranded {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let brand_name = format!("{brand}Brand", brand = self.name.0).to_lower_camel_case();

        let mut blocks = vec![];

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(state, context)?);
        }

        blocks.push(format!(
            "{indent}export type {name} = {primitive} & {{ [{brand_name}]: true }};",
            indent = state.indent_str(),
            name = self.name.render(state, context)?,
            primitive = self.primitive.render(state, context)?
        ));

        blocks.push(format!(
            "{indent}declare const {brand_name}: unique symbol;",
            indent = state.indent_str(),
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
            .render(Default::default(), &mut Default::default())
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
            .render(Default::default(), &mut Default::default())
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
            .render(
                TSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            r#"  /** Object version. */
  export type Version = number & { [versionBrand]: true };
  declare const versionBrand: unique symbol;"#
        );
    }
}
