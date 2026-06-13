use std::error::Error;
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

fn run_success(args: &[&str], stdin: &str) -> Result<String, Box<dyn Error>> {
    let mut command = Command::cargo_bin("markdown-rs-cli")?;
    let assert = command.args(args).write_stdin(stdin).assert().success();

    Ok(String::from_utf8(assert.get_output().stdout.clone())?)
}

#[test]
fn auto_converts_markdown_stdin_to_mdast_stdout() -> Result<(), Box<dyn Error>> {
    let output = run_success(&[], "# Hi\n")?;
    let json: Value = serde_json::from_str(&output)?;

    assert_eq!(json["type"], "root");

    Ok(())
}

#[test]
fn explicit_markdown_converts_markdown_stdin_to_mdast_stdout() -> Result<(), Box<dyn Error>> {
    let output = run_success(&["--from", "markdown"], "# Hi\n")?;
    let json: Value = serde_json::from_str(&output)?;

    assert_eq!(json["type"], "root");

    Ok(())
}

#[test]
fn converts_markdown_file_to_mdast_file() -> Result<(), Box<dyn Error>> {
    let directory = tempfile::tempdir()?;
    let input = directory.path().join("README.md");
    let output = directory.path().join("README.mdast.json");

    fs::write(&input, "# Hi\n")?;

    Command::cargo_bin("markdown-rs-cli")?
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .assert()
        .success();

    let json: Value = serde_json::from_str(&fs::read_to_string(&output)?)?;
    assert_eq!(json["type"], "root");

    Ok(())
}

#[test]
fn auto_converts_mdast_stdin_to_markdown_stdout() -> Result<(), Box<dyn Error>> {
    let mdast_json = run_success(&["--from", "markdown"], "# Hi\n")?;
    let markdown = run_success(&[], &mdast_json)?;

    assert_eq!(markdown, "# Hi\n");

    Ok(())
}

#[test]
fn explicit_mdast_converts_mdast_stdin_to_markdown_stdout() -> Result<(), Box<dyn Error>> {
    let mdast_json = run_success(&["--from", "markdown"], "# Hi\n")?;
    let markdown = run_success(&["--from", "mdast"], &mdast_json)?;

    assert_eq!(markdown, "# Hi\n");

    Ok(())
}

#[test]
fn converts_mdast_stdin_to_markdown_explicit_stdout() -> Result<(), Box<dyn Error>> {
    let mdast_json = run_success(&["--from", "markdown"], "# Hi\n")?;
    let markdown = run_success(&["--from", "mdast", "--output", "-"], &mdast_json)?;

    assert_eq!(markdown, "# Hi\n");

    Ok(())
}

#[test]
fn converts_mdast_file_to_markdown_file() -> Result<(), Box<dyn Error>> {
    let directory = tempfile::tempdir()?;
    let mdast_path = directory.path().join("README.mdast.json");
    let output = directory.path().join("README.restored.md");

    let mdast_json = run_success(&["--from", "markdown"], "# Hi\n")?;
    fs::write(&mdast_path, mdast_json)?;

    Command::cargo_bin("markdown-rs-cli")?
        .arg(&mdast_path)
        .arg("--output")
        .arg(&output)
        .assert()
        .success();

    assert_eq!(fs::read_to_string(&output)?, "# Hi\n");

    Ok(())
}

#[test]
fn explicit_markdown_forces_json_looking_input_to_markdown() -> Result<(), Box<dyn Error>> {
    let output = run_success(
        &["--from", "markdown"],
        "{\"type\":\"root\",\"children\":[]}\n",
    )?;
    let json: Value = serde_json::from_str(&output)?;

    assert_eq!(json["type"], "root");
    assert_eq!(json["children"][0]["type"], "paragraph");

    Ok(())
}

#[test]
fn rejects_invalid_mdast_json() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin("markdown-rs-cli")?
        .arg("--from")
        .arg("mdast")
        .write_stdin("{not-json")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "input is not valid markdown-rs mdast JSON",
        ));

    Ok(())
}

#[test]
fn rejects_oversized_input() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin("markdown-rs-cli")?
        .arg("--max-bytes")
        .arg("3")
        .write_stdin("1234")
        .assert()
        .failure()
        .stderr(predicate::str::contains("input exceeds --max-bytes limit"));

    Ok(())
}

#[test]
fn mdast_to_markdown_does_not_escape_url_underscores() -> Result<(), Box<dyn Error>> {
    let input = r#"### GitLab

* CI/CD variables:
  https://docs.gitlab.com/ci/variables/

* CI/CD job tokens:
  https://docs.gitlab.com/ci/jobs/ci_job_token/

* Git submodules in CI/CD:
  https://docs.gitlab.com/ci/runners/git_submodules/

* Protected branches:
  https://docs.gitlab.com/user/project/repository/branches/protected/

* Protected runners:
  https://docs.gitlab.com/ci/runners/configure_runners/

* Protected environments and tier availability:
  https://docs.gitlab.com/ci/environments/protected_environments/

* Project API job-token push setting:
  https://docs.gitlab.com/api/projects/
"#;

    let mdast_json = run_success(&["--from", "markdown", "--dialect", "gfm"], input)?;
    let markdown = run_success(&["--from", "mdast"], &mdast_json)?;

    assert!(markdown.contains("https://docs.gitlab.com/ci/jobs/ci_job_token/"));
    assert!(markdown.contains("https://docs.gitlab.com/ci/runners/git_submodules/"));
    assert!(markdown.contains("https://docs.gitlab.com/ci/runners/configure_runners/"));
    assert!(markdown.contains("https://docs.gitlab.com/ci/environments/protected_environments/"));

    assert!(!markdown.contains(r"ci\_job\_token"));
    assert!(!markdown.contains(r"git\_submodules"));
    assert!(!markdown.contains(r"configure\_runners"));
    assert!(!markdown.contains(r"protected\_environments"));

    Ok(())
}
