use crate::prelude::internal::*;

impl TSDoc {
    pub fn with_doc(
        doc: &Option<TSDoc>,
        state: TSRenderState,
        context: &mut TSRenderContext,
        str: String,
        padded: bool,
    ) -> Result<String> {
        let doc = if let Some(doc) = doc {
            doc.render(state, context)? + if padded { "\n\n" } else { "\n" }
        } else {
            String::new()
        };
        Ok(format!("{doc}{str}"))
    }
}

impl<'a> GtlRender<'a> for TSDoc {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        let lines = self.0.split("\n").enumerate();
        Ok(lines
            .map(|(index, line)| {
                format!(
                    "{}{} {}",
                    state.indent_str(),
                    if index == 0 { "/**" } else { " *" },
                    line
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
            + " */")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_simple() {
        assert_eq!(
            TSDoc("Hello, world!".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "/** Hello, world! */"
        );
    }

    #[test]
    fn test_render_multiline() {
        assert_eq!(
            TSDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            )
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"/** Hello,
 * cruel
 * world! */"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            TSDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            )
            .render(
                TSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            r#"  /** Hello,
   * cruel
   * world! */"#
        );
    }

    #[test]
    fn test_with_doc_some() {
        assert_eq!(
            TSDoc::with_doc(
                &Some(TSDoc("Hello, world!".into())),
                Default::default(),
                &mut Default::default(),
                "type Name = string;".into(),
                false
            )
            .unwrap(),
            r#"/** Hello, world! */
type Name = string;"#
        );
    }

    #[test]
    fn test_with_doc_none() {
        assert_eq!(
            TSDoc::with_doc(
                &None,
                Default::default(),
                &mut Default::default(),
                "type Name = string;".into(),
                false
            )
            .unwrap(),
            r#"type Name = string;"#
        );
    }

    #[test]
    fn test_with_doc_padded_some() {
        assert_eq!(
            TSDoc::with_doc(
                &Some(TSDoc("Hello, world!".into())),
                Default::default(),
                &mut Default::default(),
                "type Name = string;".into(),
                true
            )
            .unwrap(),
            r#"/** Hello, world! */

type Name = string;"#
        );
    }

    #[test]
    fn test_with_doc_padded_none() {
        assert_eq!(
            TSDoc::with_doc(
                &None,
                Default::default(),
                &mut Default::default(),
                "type Name = string;".into(),
                true
            )
            .unwrap(),
            r#"type Name = string;"#
        );
    }
}
