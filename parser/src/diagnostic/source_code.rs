use miette::SourceCode;

#[derive(Debug, Clone, PartialEq)]
pub struct GTSourceCode {
    pub name: String,
    pub content: String,
}

impl GTSourceCode {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
}

impl SourceCode for GTSourceCode {
    fn read_span<'a>(
        &'a self,
        span: &miette::SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        SourceCode::read_span(
            &self.content,
            span,
            context_lines_before,
            context_lines_after,
        )
    }
}
