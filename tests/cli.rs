use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn emits_json_to_stdout_from_stdin() {
    let mut command = Command::cargo_bin("markdown-rs-cli").expect("binary should exist");

    command
        .write_stdin("# Title\n\nText.\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Title"))
        .stdout(predicate::str::contains("Text"));
}

#[test]
fn emits_json_file_from_input_file() {
    let input = NamedTempFile::new().expect("input temp file should be created");

    let output = NamedTempFile::new().expect("output temp file should be created");

    std::fs::write(input.path(), "# Title\n\n- [x] done\n")
        .expect("input temp file should be writable");

    let mut command = Command::cargo_bin("markdown-rs-cli").expect("binary should exist");

    command
        .arg("--dialect")
        .arg("gfm")
        .arg("--output")
        .arg(output.path())
        .arg(input.path())
        .assert()
        .success();

    let json = std::fs::read_to_string(output.path()).expect("output JSON should be readable");

    assert!(json.contains("Title"));
    assert!(json.contains("done"));
}

#[test]
fn rejects_too_large_input() {
    let mut command = Command::cargo_bin("markdown-rs-cli").expect("binary should exist");

    command
        .arg("--max-bytes")
        .arg("3")
        .write_stdin("1234")
        .assert()
        .failure()
        .stderr(predicate::str::contains("exceeds --max-bytes"));
}
