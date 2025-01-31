use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tokio::test;

#[test]
async fn test_cli_async_template_generation() {
    // Clean up any existing directories from previous failed tests
    let _ = fs::remove_dir_all("test");

    let mut cmd = Command::cargo_bin("essex").unwrap();
    let assert = cmd
        .arg("new")
        .arg("basic")  // template name
        .arg("test/project")  // project name
        .assert();

    assert.success();

    // Verify essential files exist in current directory
    let dockerfile = std::path::Path::new("test/project/Dockerfile");
    assert!(dockerfile.exists());
    let makefile = std::path::Path::new("test/project/Makefile");
    assert!(makefile.exists());
    let readme = std::path::Path::new("test/project/README.md");
    assert!(readme.exists());
    let entrypoint =
        std::path::Path::new("test/project/runtime-assets/usr/local/bin/entrypoint.sh");
    assert!(entrypoint.exists());

    // Clean up
    fs::remove_dir_all("test").unwrap();
}

#[test]
async fn test_cli_async_template_with_options() {
    // Clean up any existing directories from previous failed tests
    let _ = fs::remove_dir_all("custom");

    let mut cmd = Command::cargo_bin("essex").unwrap();
    let assert = cmd
        .arg("new")
        .arg("basic")  // template name
        .arg("custom/project")  // project name
        .arg("--username")
        .arg("customuser")
        .arg("--vendor")
        .arg("Custom Corp")
        .assert();

    assert.success();

    // Verify file contents with custom values
    let dockerfile_path = std::path::Path::new("custom/project/Dockerfile");
    assert!(dockerfile_path.exists());
    let dockerfile_content = fs::read_to_string(dockerfile_path).unwrap();
    assert!(dockerfile_content.contains("org.opencontainers.image.vendor=\"Custom Corp\""));
    assert!(dockerfile_content
        .contains("org.opencontainers.image.authors=\"customuser <contact@example.com>\""));

    let makefile_path = std::path::Path::new("custom/project/Makefile");
    assert!(makefile_path.exists());
    let makefile_content = fs::read_to_string(makefile_path).unwrap();
    assert!(makefile_content.contains("REPO_USERNAME         ?= customuser"));

    // Clean up
    fs::remove_dir_all("custom").unwrap();
}

#[test]
async fn test_cli_async_template_error_handling() {
    // Test invalid project name
    let mut cmd = Command::cargo_bin("essex").unwrap();
    cmd.arg("new")
        .arg("basic")  // template name
        .arg("invalid-project-name")  // Missing namespace
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error: InvalidProjectName"));

    // Test non-existent template
    let mut cmd = Command::cargo_bin("essex").unwrap();
    cmd.arg("new")
        .arg("non-existent")  // non-existent template
        .arg("test/project")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error: TemplateNotFound"));
}

#[test]
async fn test_cli_completion_generation() {
    // Test bash completion
    let mut cmd = Command::cargo_bin("essex").unwrap();
    cmd.arg("completion")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::contains("complete -F"));

    // Test zsh completion
    let mut cmd = Command::cargo_bin("essex").unwrap();
    cmd.arg("completion")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef essex"));
}
