use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

fn setup_test_dir() -> assert_fs::TempDir {
    assert_fs::TempDir::new().unwrap()
}

#[test]
fn test_list_command() {
    let temp = setup_test_dir();

    let mut cmd = Command::cargo_bin("essex").unwrap();
    let assert = cmd.current_dir(&temp).arg("list").assert();

    assert
        .success()
        .stdout(predicate::str::contains("Available templates:"))
        .stdout(predicate::str::contains("basic"));
}

#[test]
fn test_new_command() {
    let temp = assert_fs::TempDir::new().unwrap();
    let test_dir = temp.child("test");
    test_dir.create_dir_all().unwrap();

    Command::cargo_bin("essex")
        .unwrap()
        .current_dir(test_dir.path())
        .arg("new")
        .arg("basic")
        .arg("testuser/myproject")
        .assert()
        .success();

    let project_dir = test_dir.child("testuser/myproject");
    project_dir
        .child("Dockerfile")
        .assert(predicate::path::exists());
    project_dir
        .child("Makefile")
        .assert(predicate::path::exists());
    project_dir
        .child("README.md")
        .assert(predicate::path::exists());
    project_dir
        .child("runtime-assets")
        .assert(predicate::path::exists());
}

#[test]
fn test_invalid_template() {
    let temp = setup_test_dir();

    let mut cmd = Command::cargo_bin("essex").unwrap();
    let assert = cmd
        .current_dir(&temp)
        .args(["new", "nonexistent", "test/myproject"])
        .assert();

    assert
        .failure()
        .stderr(predicate::str::contains("TemplateNotFound"));
}
