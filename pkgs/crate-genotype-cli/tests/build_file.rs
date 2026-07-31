use insta::assert_snapshot;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Output};
use tempfile::{TempDir, tempdir};

#[test]
fn removes_stale_dist_files() {
    let (
        _tempdir,
        PreparedProject {
            project_dir,
            build_info_path,
            stale_src_path,
            stale_dist_path,
            keep_src_path,
            ..
        },
    ) = prepare_project();

    // First build

    assert_build_success(&build(&project_dir));

    assert_snapshot!(fs::read_to_string(&build_info_path).unwrap(), @r#"
    [src]
    config_hash = "948e19e0a12a5a4b"

    [src.modules."src/keep.type"]
    id = "keep"
    hash = "a702d52582093a15"
    deps = ["stale"]

    [src.modules."src/stale.type"]
    id = "stale"
    hash = "6acbfbcc23ce65cb"
    deps = []

    [dist.ts."dist/ts/index.ts"]
    hash = "8cadd476d24cae2a"

    [dist.ts."dist/ts/keep.ts"]
    hash = "3f4324d7b2333299"
    srcId = "keep"

    [dist.ts."dist/ts/stale.ts"]
    hash = "71926b14eaa99dde"
    srcId = "stale"
    "#);

    assert!(stale_dist_path.is_file());

    // Remove `stale.type` and rebuild

    fs::write(&keep_src_path, "Keep: { value: string }").unwrap();
    fs::remove_file(&stale_src_path).unwrap();

    assert_build_success(&build(&project_dir));

    assert_snapshot!(fs::read_to_string(&build_info_path).unwrap(), @r#"
    [src]
    config_hash = "948e19e0a12a5a4b"

    [src.modules."src/keep.type"]
    id = "keep"
    hash = "302a2c2dfed7c5b7"
    deps = []

    [dist.ts."dist/ts/index.ts"]
    hash = "693840cae3a4b09c"

    [dist.ts."dist/ts/keep.ts"]
    hash = "84c345da19059149"
    srcId = "keep"
    "#);

    assert!(!stale_dist_path.exists());
}

#[test]
fn preserves_and_warns_about_changed_stale_dist_files() {
    let (
        _tempdir,
        PreparedProject {
            project_dir,
            build_info_path,
            stale_src_path,
            stale_dist_path,
            keep_src_path,
            ..
        },
    ) = prepare_project();

    // First build

    assert_build_success(&build(&project_dir));

    assert_snapshot!(fs::read_to_string(&build_info_path).unwrap(), @r#"
    [src]
    config_hash = "948e19e0a12a5a4b"

    [src.modules."src/keep.type"]
    id = "keep"
    hash = "a702d52582093a15"
    deps = ["stale"]

    [src.modules."src/stale.type"]
    id = "stale"
    hash = "6acbfbcc23ce65cb"
    deps = []

    [dist.ts."dist/ts/index.ts"]
    hash = "8cadd476d24cae2a"

    [dist.ts."dist/ts/keep.ts"]
    hash = "3f4324d7b2333299"
    srcId = "keep"

    [dist.ts."dist/ts/stale.ts"]
    hash = "71926b14eaa99dde"
    srcId = "stale"
    "#);

    assert!(stale_dist_path.is_file());

    // Remove `stale.type`, change the contents of `stale.ts`, and rebuild

    fs::write(&keep_src_path, "Keep: { value: string }").unwrap();
    fs::remove_file(&stale_src_path).unwrap();
    fs::write(&stale_dist_path, "// user edit\n").unwrap();

    let output = build(&project_dir);
    assert_build_success(&output);

    assert_snapshot!(fs::read_to_string(&build_info_path).unwrap(), @r#"
    [src]
    config_hash = "948e19e0a12a5a4b"

    [src.modules."src/keep.type"]
    id = "keep"
    hash = "302a2c2dfed7c5b7"
    deps = []

    [dist.ts."dist/ts/index.ts"]
    hash = "693840cae3a4b09c"

    [dist.ts."dist/ts/keep.ts"]
    hash = "84c345da19059149"
    srcId = "keep"
    "#);

    assert!(String::from_utf8_lossy(&output.stderr).contains("contents changed"));
    assert!(stale_dist_path.is_file());
}

#[test]
fn preserves_stale_files_when_clean_up_disabled() {
    let (
        _tempdir,
        PreparedProject {
            project_dir,
            config_path,
            build_info_path,
            keep_src_path,
            stale_src_path,
            stale_dist_path,
            ..
        },
    ) = prepare_project();

    fs::write(
        &config_path,
        "[build]\ncleanup = false\n[ts]\nenabled = true\npackage = false\n",
    )
    .unwrap();

    // First build

    assert_build_success(&build(&project_dir));

    assert_snapshot!(fs::read_to_string(&build_info_path).unwrap(), @r#"
    [src]
    config_hash = "6a00f7d6a6a5a62a"

    [src.modules."src/keep.type"]
    id = "keep"
    hash = "a702d52582093a15"
    deps = ["stale"]

    [src.modules."src/stale.type"]
    id = "stale"
    hash = "6acbfbcc23ce65cb"
    deps = []

    [dist.ts."dist/ts/index.ts"]
    hash = "8cadd476d24cae2a"

    [dist.ts."dist/ts/keep.ts"]
    hash = "3f4324d7b2333299"
    srcId = "keep"

    [dist.ts."dist/ts/stale.ts"]
    hash = "71926b14eaa99dde"
    srcId = "stale"
    "#);

    assert!(stale_dist_path.is_file());

    // Remove `stale.type` and rebuild

    fs::write(&keep_src_path, "Keep: { value: string }").unwrap();
    fs::remove_file(&stale_src_path).unwrap();

    assert_build_success(&build(&project_dir));

    assert_snapshot!(fs::read_to_string(&build_info_path).unwrap(), @r#"
    [src]
    config_hash = "6a00f7d6a6a5a62a"

    [src.modules."src/keep.type"]
    id = "keep"
    hash = "302a2c2dfed7c5b7"
    deps = []

    [dist.ts."dist/ts/index.ts"]
    hash = "693840cae3a4b09c"

    [dist.ts."dist/ts/keep.ts"]
    hash = "84c345da19059149"
    srcId = "keep"
    "#);

    assert!(stale_dist_path.is_file());
}

#[test]
fn warns_when_cleanup_is_enabled_without_build_file() {
    let (
        _tempdir,
        PreparedProject {
            project_dir,
            config_path,
            build_info_path,
            ..
        },
    ) = prepare_project();

    fs::write(
        &config_path,
        "[build]\nfile = false\ncleanup = true\n[ts]\nenabled = true\npackage = false\n",
    )
    .unwrap();

    let output = build(&project_dir);
    assert_build_success(&output);

    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("`build.cleanup` has no effect when `build.file` is disabled")
    );

    assert!(!build_info_path.exists());
}

struct PreparedProject {
    project_dir: PathBuf,
    config_path: PathBuf,
    build_info_path: PathBuf,
    keep_src_path: PathBuf,
    stale_src_path: PathBuf,
    stale_dist_path: PathBuf,
}

fn prepare_project() -> (TempDir, PreparedProject) {
    let project_tempdir = tempdir().unwrap();
    let project_dir = project_tempdir.path().to_path_buf();

    let config_path = project_dir.join("genotype.toml");
    let build_info_path = project_dir.join("genotype.build.toml");
    let keep_src_path = project_dir.join("src/keep.type");
    let stale_src_path = project_dir.join("src/stale.type");
    let stale_dist_path = project_dir.join("dist/ts/stale.ts");

    fs::create_dir(project_dir.join("src")).unwrap();

    fs::write(&config_path, "[ts]\nenabled = true\npackage = false\n").unwrap();

    fs::write(&keep_src_path, "use ./stale/Stale\nKeep: { stale: Stale }").unwrap();

    fs::write(&stale_src_path, "Stale: { value: string }").unwrap();

    (
        project_tempdir,
        PreparedProject {
            project_dir,
            config_path,
            build_info_path,
            keep_src_path,
            stale_src_path,
            stale_dist_path,
        },
    )
}

fn build(project_dir: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_gt"))
        .arg("build")
        .arg(project_dir)
        .output()
        .unwrap()
}

fn assert_build_success(output: &Output) {
    assert!(
        output.status.success(),
        "gt build failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
