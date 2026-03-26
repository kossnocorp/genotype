use std::process::Command;

use insta::assert_snapshot;

#[test]
fn running_without_arguments_prints_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_gt")).output().unwrap();
    assert!(
        output.status.success(),
        "gt failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_snapshot!(
        stdout,
        @"
    Genotype language CLI

    Usage: gt [COMMAND]

    Commands:
      build    Builds a Genotype project
      init     Initializes a Genotype project
      version  Manage package versions in genotype.toml
      help     Print this message or the help of the given subcommand(s)

    Options:
      -h, --help     Print help
      -V, --version  Print version
    "
    );
}
