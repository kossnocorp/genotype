use std::path::PathBuf;
use std::process::Command;

#[test]
fn recursive_box_generated_rust_compiles() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let example_dir = manifest_dir.join("examples/recursive-box");
    let dist_dir = example_dir.join("dist");

    if dist_dir.exists() {
        std::fs::remove_dir_all(&dist_dir).unwrap();
    }

    let output = Command::new(env!("CARGO_BIN_EXE_gt"))
        .arg("build")
        .arg("examples/recursive-box")
        .current_dir(&manifest_dir)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "gt build failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    unsafe {
        std::env::set_var(
            "GENOTYPE_CLI_MANIFEST_DIR",
            manifest_dir.to_string_lossy().as_ref(),
        );
    }

    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/recursive_box_generated.rs");
}
