use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsDoc {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(self
            .0
            .split("\n")
            .map(|line| {
                state.indent_format(&format!(
                    r#"{} {}"#,
                    if self.1 { "//!" } else { "///" },
                    line
                ))
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_simple() {
        assert_snapshot!(
            RsDoc::new("Hello, world!", false)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"/// Hello, world!"
        );
    }

    #[test]
    fn test_render_module() {
        assert_snapshot!(
            RsDoc::new("Hello, world!", true)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"//! Hello, world!"
        );
    }

    #[test]
    fn test_render_multiline() {
        assert_snapshot!(
            RsDoc::new(
                r#"Hello,
cruel
world!"#,
                false
            )
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        /// Hello,
        /// cruel
        /// world!
        "
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            RsDoc::new(
                r#"Hello,
cruel
world!"#,
                false
            )
            .render(
                RsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        /// Hello,
        /// cruel
        /// world!
        "
        );
    }
}
