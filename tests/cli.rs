use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs;

fn setup_test_dir() -> assert_fs::TempDir {
    let temp = assert_fs::TempDir::new().unwrap();
    
    // Create templates directory
    let templates = temp.child("templates");
    templates.create_dir_all().unwrap();
    
    // Copy the basic template
    let src_templates = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates/basic");
    let dest_templates = templates.child("basic");
    fs::create_dir_all(&dest_templates).unwrap();
    
    copy_dir_all(src_templates, dest_templates.path()).unwrap();
    
    temp
}

fn copy_dir_all(src: std::path::PathBuf, dst: &std::path::Path) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            fs::create_dir_all(&dst_path)?;
            copy_dir_all(entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), &dst_path)?;
            if dst_path.file_name().unwrap() == "entrypoint.sh" {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    fs::set_permissions(&dst_path, fs::Permissions::from_mode(0o755))?;
                }
            }
        }
    }
    Ok(())
}

#[test]
fn test_list_command() {
    let temp = setup_test_dir();

    let mut cmd = Command::cargo_bin("essex").unwrap();
    let assert = cmd.current_dir(&temp)
        .arg("list")
        .assert();
    
    assert
        .success()
        .stdout(predicate::str::contains("Available templates:"))
        .stdout(predicate::str::contains("basic"));
}

#[test]
fn test_new_command() {
    let temp = setup_test_dir();
    let project_dir = temp.child("test/myproject");

    let mut cmd = Command::cargo_bin("essex").unwrap();
    let assert = cmd.current_dir(&temp)
        .args(["new", "basic", "test/myproject", "--username", "testuser", "--vendor", "TestCo"])
        .assert();
    
    assert.success()
        .stdout(predicate::str::contains("Creating new project 'test/myproject'"))
        .stdout(predicate::str::contains("Project created successfully!"));

    project_dir.child("Dockerfile").assert(predicate::path::exists());
    project_dir.child("Makefile").assert(predicate::path::exists());
    project_dir.child("README.md").assert(predicate::path::exists());
    project_dir.child("runtime-assets/usr/local/bin/entrypoint.sh").assert(predicate::path::exists());
}

#[test]
fn test_invalid_template() {
    let temp = setup_test_dir();

    let mut cmd = Command::cargo_bin("essex").unwrap();
    let assert = cmd.current_dir(&temp)
        .args(["new", "nonexistent", "test/myproject"])
        .assert();
    
    assert.failure()
        .stderr(predicate::str::contains("TemplateNotFound"));
}