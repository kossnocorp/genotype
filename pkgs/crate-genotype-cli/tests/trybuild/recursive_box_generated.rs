mod generated_node {
    include!(concat!(
        env!("GENOTYPE_CLI_MANIFEST_DIR"),
        "/examples/recursive-box/dist/rs/src/node.rs"
    ));
}

mod generated_tree {
    include!(concat!(
        env!("GENOTYPE_CLI_MANIFEST_DIR"),
        "/examples/recursive-box/dist/rs/src/tree.rs"
    ));
}

fn main() {}
