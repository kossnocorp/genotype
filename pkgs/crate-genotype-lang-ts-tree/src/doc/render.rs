use crate::prelude::internal::*;

impl TsDoc {
    pub fn with_doc(
        doc: &Option<TsDoc>,
        state: TsRenderState,
        context: &mut TsRenderContext,
        str: String,
        padded: bool,
    ) -> TsRenderResult<String> {
        let doc = if let Some(doc) = doc {
            doc.render(state, context)? + if padded { "\n\n" } else { "\n" }
        } else {
            String::new()
        };
        Ok(format!("{doc}{str}"))
    }
}

impl<'context> GtlRender<'context, TsRenderTypes> for TsDoc {
    fn render(
        &self,
        state: TsRenderState,
        _context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
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
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_simple() {
        assert_snapshot!(
            render_node(Tst::doc("Hello, world!")),
            @"/** Hello, world! */"
        );
    }

    #[test]
    fn test_render_multiline() {
        assert_snapshot!(
            render_node(
                TsDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            ),
            ),
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
        let mut context = Tst::render_context();
        assert_snapshot!(
            TsDoc::with_doc(
                &Tst::some_doc("Hello, world!"),
                Default::default(),
                &mut context,
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
        let mut context = Tst::render_context();
        assert_snapshot!(
            TsDoc::with_doc(
                &None,
                Default::default(),
                &mut context,
                "type Name = string;".into(),
                false
            )
            .unwrap(),
            @"type Name = string;"
        );
    }

    #[test]
    fn test_with_doc_padded_some() {
        let mut context = Tst::render_context();
        assert_snapshot!(
            TsDoc::with_doc(
                &Tst::some_doc("Hello, world!"),
                Default::default(),
                &mut context,
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
        let mut context = Tst::render_context();
        assert_snapshot!(
            TsDoc::with_doc(
                &None,
                Default::default(),
                &mut context,
                "type Name = string;".into(),
                true
            )
            .unwrap(),
            @"type Name = string;"
        );
    }
}
