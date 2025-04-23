use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSDoc {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_simple() {
        assert_eq!(
            RSDoc::new("Hello, world!", false)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            r#"/// Hello, world!"#
        );
    }

    #[test]
    fn test_render_module() {
        assert_eq!(
            RSDoc::new("Hello, world!", true)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            r#"//! Hello, world!"#
        );
    }

    #[test]
    fn test_render_multiline() {
        assert_eq!(
            RSDoc::new(
                r#"Hello,
cruel
world!"#,
                false
            )
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"/// Hello,
/// cruel
/// world!"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSDoc::new(
                r#"Hello,
cruel
world!"#,
                false
            )
            .render(
                RSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            r#"    /// Hello,
    /// cruel
    /// world!"#
        );
    }
}
