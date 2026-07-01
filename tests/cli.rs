use std::process::Command;

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
    assert_eq!(String::from_utf8_lossy(&output.stdout), "codexa 0.0.1\n");
}
