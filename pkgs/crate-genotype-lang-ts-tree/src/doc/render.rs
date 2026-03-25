use crate::prelude::internal::*;

impl TsDoc {
    pub fn with_doc(
        doc: &Option<TsDoc>,
        state: TsRenderState,
        context: &mut TsRenderContext,
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

impl<'a> GtlRender<'a> for TsDoc {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

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
    use insta::assert_snapshot;

    #[test]
    fn test_render_simple() {
        assert_snapshot!(
            TsDoc("Hello, world!".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"/** Hello, world! */"
        );
    }

    #[test]
    fn test_render_multiline() {
        assert_snapshot!(
            TsDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            )
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        /** Hello,
         * cruel
         * world! */
        "
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            TsDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            )
            .render(
                TsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        /** Hello,
         * cruel
         * world! */
        "
        );
    }

    #[test]
    fn test_with_doc_some() {
        assert_snapshot!(
            TsDoc::with_doc(
                &Some(TsDoc("Hello, world!".into())),
                Default::default(),
                &mut Default::default(),
                "type Name = string;".into(),
                false
            )
            .unwrap(),
            @"
        /** Hello, world! */
        type Name = string;
        "
        );
    }

    #[test]
    fn test_with_doc_none() {
        assert_snapshot!(
            TsDoc::with_doc(
                &None,
                Default::default(),
                &mut Default::default(),
                "type Name = string;".into(),
                false
            )
            .unwrap(),
            @"type Name = string;"
        );
    }

    #[test]
    fn test_with_doc_padded_some() {
        assert_snapshot!(
            TsDoc::with_doc(
                &Some(TsDoc("Hello, world!".into())),
                Default::default(),
                &mut Default::default(),
                "type Name = string;".into(),
                true
            )
            .unwrap(),
            @"
        /** Hello, world! */

        type Name = string;
        "
        );
    }

    #[test]
    fn test_with_doc_padded_none() {
        assert_snapshot!(
            TsDoc::with_doc(
                &None,
                Default::default(),
                &mut Default::default(),
                "type Name = string;".into(),
                true
            )
            .unwrap(),
            @"type Name = string;"
        );
    }
}
