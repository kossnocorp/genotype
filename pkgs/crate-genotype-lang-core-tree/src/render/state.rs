pub trait GtlRenderState: Copy {
    const INDENT: &'static str = "  ";

    fn indent_inc(&self) -> Self;

    fn indent_level(&self) -> usize;

    fn indent_str(&self) -> String {
        Self::INDENT.repeat(self.indent_level())
    }

    fn indent_format(&self, str: &str) -> String {
        let indent = self.indent_str();
        format!("{indent}{str}")
    }
}
