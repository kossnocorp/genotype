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
