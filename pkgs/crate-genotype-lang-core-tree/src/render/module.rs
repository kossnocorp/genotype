pub trait GtlRenderModule {
    fn join_imports(imports: &[String]) -> String {
        imports.join("\n")
    }

    fn join_definitions(definitions: &[String]) -> String {
        definitions.join("\n\n")
    }

    fn join_blocks(blocks: &[String]) -> String {
        blocks.join("\n\n") + "\n"
    }
}
