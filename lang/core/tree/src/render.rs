use crate::indent::GTIndent;

pub trait GTRender {
    fn render(&self, indent: &GTIndent) -> String;
}

pub trait GTRenderModule {
    fn join_imports(imports: Vec<String>) -> String {
        imports.join("\n")
    }

    fn join_definitions(definitions: Vec<String>) -> String {
        definitions.join("\n\n")
    }

    fn join_blocks(blocks: Vec<String>) -> String {
        blocks.join("\n\n") + "\n"
    }
}
