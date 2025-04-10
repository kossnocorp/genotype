use miette::Result;

pub trait GtlRender<'a> {
    type RenderContext: GtlRenderContext;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String>;
}

pub trait GtlRenderContext {
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

pub trait GtlRenderModule {
    fn join_imports(imports: &Vec<String>) -> String {
        imports.join("\n")
    }

    fn join_definitions(definitions: &Vec<String>) -> String {
        definitions.join("\n\n")
    }

    fn join_blocks(blocks: &Vec<String>) -> String {
        blocks.join("\n\n") + "\n"
    }
}
