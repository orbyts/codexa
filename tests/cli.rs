use std::{fs, process::Command};

#[test]
fn prints_hello_world() {
    let output = Command::new(env!("CARGO_BIN_EXE_codexa"))
        .output()
        .expect("codexa binary should run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Hello from Codexa!\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn prints_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_codexa"))
        .arg("--version")
        .output()
        .expect("codexa binary should run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("codexa {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn builds_a_web_artifact() {
    let temp = tempfile::tempdir().expect("temporary directory should be created");
    let input = temp.path().join("sample.md");
    let output_dir = temp.path().join("web");
    fs::write(&input, "Hello from Codexa web.\n").expect("fixture should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_codexa"))
        .args([
            "build",
            input.to_str().expect("input path should be UTF-8"),
            "--adapter",
            "web",
            "--output",
            output_dir.to_str().expect("output path should be UTF-8"),
        ])
        .output()
        .expect("codexa binary should run");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_dir.join("manifest.json").is_file());
    assert!(output_dir.join("documents/sample.json").is_file());
}
