use crate::prelude::internal::*;
use heck::ToLowerCamelCase;

impl<'a> GtlRender<'a> for TsBranded {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

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
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            TsBranded {
                doc: None,
                name: "Version".into(),
                primitive: TsPrimitive::Number
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        export type Version = number & { [versionBrand]: true };
        declare const versionBrand: unique symbol;
        "
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            TsBranded {
                doc: Some(TsDoc("Object version.".into())),
                name: "Version".into(),
                primitive: TsPrimitive::Number
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        /** Object version. */
        export type Version = number & { [versionBrand]: true };
        declare const versionBrand: unique symbol;
        "
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            TsBranded {
                doc: Some(TsDoc("Object version.".into())),
                name: "Version".into(),
                primitive: TsPrimitive::Number
            }
            .render(
                TsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        /** Object version. */
        export type Version = number & { [versionBrand]: true };
        declare const versionBrand: unique symbol;
        "
        );
    }
}
