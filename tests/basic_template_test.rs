use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use std::process::Command as StdCommand;
use tempfile::TempDir;

fn setup_git_config(project_dir: &PathBuf) {
    // Try local config first
    if let Err(_) = StdCommand::new("git")
        .args(["config", "--local", "user.email", "test@example.com"])
        .current_dir(project_dir)
        .output()
    {
        // Fallback to global config if local fails
        StdCommand::new("git")
            .args(["config", "--global", "user.email", "test@example.com"])
            .output()
            .expect("Failed to set git email");

        StdCommand::new("git")
            .args(["config", "--global", "user.name", "Test User"])
            .output()
            .expect("Failed to set git name");
    } else {
        StdCommand::new("git")
            .args(["config", "--local", "user.name", "Test User"])
            .current_dir(project_dir)
            .output()
            .expect("Failed to set git name");
    }
}

fn setup_test_project() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let output = Command::cargo_bin("essex")
        .unwrap()
        .args([
            "new",
            "basic",
            "testuser/test-project",
            "--username",
            "testuser",
            "--vendor",
            "Test Company",
        ])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let project_dir = temp_dir.path().join("testuser").join("test-project");
    assert!(
        project_dir.exists(),
        "Project directory was not created at {:?}",
        project_dir
    );

    // Initialize git repository
    let git_init = StdCommand::new("git")
        .args(["init"])
        .current_dir(&project_dir)
        .output()
        .expect("Failed to initialize git repository");

    assert!(
        git_init.status.success(),
        "Git init failed: {}",
        String::from_utf8_lossy(&git_init.stderr)
    );

    // Setup git configuration
    setup_git_config(&project_dir);

    // Add files to git
    let git_add = StdCommand::new("git")
        .args(["add", "."])
        .current_dir(&project_dir)
        .output()
        .expect("Failed to add files to git");

    assert!(
        git_add.status.success(),
        "Git add failed: {}",
        String::from_utf8_lossy(&git_add.stderr)
    );

    // Initial commit
    let git_commit = StdCommand::new("git")
        .args([
            "-c",
            "user.email=test@example.com",
            "-c",
            "user.name=Test User",
            "commit",
            "-m",
            "Initial commit",
        ])
        .current_dir(&project_dir)
        .output()
        .expect("Failed to commit files");

    assert!(
        git_commit.status.success(),
        "Git commit failed: {}",
        String::from_utf8_lossy(&git_commit.stderr)
    );

    // Create an initial tag
    let git_tag = StdCommand::new("git")
        .args([
            "-c",
            "user.email=test@example.com",
            "-c",
            "user.name=Test User",
            "tag",
            "-a",
            "v0.1.0",
            "-m",
            "Initial release",
        ])
        .current_dir(&project_dir)
        .output()
        .expect("Failed to create git tag");

    assert!(
        git_tag.status.success(),
        "Git tag failed: {}",
        String::from_utf8_lossy(&git_tag.stderr)
    );

    (temp_dir, project_dir)
}

fn is_ci() -> bool {
    std::env::var("CI").is_ok()
}

#[test]
fn test_template_generation() {
    let (_temp_dir, project_dir) = setup_test_project();

    // Check if all required files are generated
    let required_files = vec![
        "Dockerfile",
        "Makefile",
        "README.md",
        "runtime-assets/usr/local/bin/entrypoint.sh",
    ];

    for file in required_files {
        let file_path = project_dir.join(file);
        assert!(
            file_path.exists(),
            "Missing required file: {} at {:?}",
            file,
            file_path
        );
    }

    // Verify file permissions
    let entrypoint_path = project_dir.join("runtime-assets/usr/local/bin/entrypoint.sh");
    let metadata = fs::metadata(&entrypoint_path)
        .unwrap_or_else(|e| panic!("Failed to get metadata for {:?}: {}", entrypoint_path, e));

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert!(
            metadata.permissions().mode() & 0o111 != 0,
            "entrypoint.sh should be executable"
        );
    }
}

#[test]
#[cfg_attr(not(feature = "docker_tests"), ignore)]
fn test_dockerfile_contents() {
    if is_ci() {
        println!("Skipping Docker tests in CI environment");
        return;
    }
    let (_temp_dir, project_dir) = setup_test_project();
    let dockerfile_path = project_dir.join("Dockerfile");
    let dockerfile_content = fs::read_to_string(&dockerfile_path)
        .unwrap_or_else(|e| panic!("Failed to read Dockerfile at {:?}: {}", dockerfile_path, e));

    // Check for required Dockerfile elements
    assert!(dockerfile_content.contains("ARG BASE_IMAGE=alpine:3.21"));
    assert!(dockerfile_content.contains("FROM ${BASE_IMAGE}"));
    assert!(dockerfile_content.contains("addgroup -g 1000 -S essex"));
    assert!(dockerfile_content.contains("adduser -u 1000 -S -h /essex"));
    assert!(dockerfile_content.contains("org.opencontainers.image.authors=\"testuser"));
    assert!(dockerfile_content.contains("org.opencontainers.image.vendor=\"Test Company\""));
}

#[test]
#[cfg_attr(not(feature = "docker_tests"), ignore)]
fn test_makefile_functionality() {
    if is_ci() {
        println!("Skipping Docker tests in CI environment");
        return;
    }
    let (_temp_dir, project_dir) = setup_test_project();

    // Test make build
    let build_output = StdCommand::new("make")
        .arg("build")
        .current_dir(&project_dir)
        .output()
        .unwrap_or_else(|e| panic!("Failed to execute make build: {}", e));

    assert!(
        build_output.status.success(),
        "Make build failed: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    // Test make list
    let list_output = StdCommand::new("make")
        .arg("list")
        .current_dir(&project_dir)
        .output()
        .unwrap_or_else(|e| panic!("Failed to execute make list: {}", e));

    assert!(
        list_output.status.success(),
        "Make list failed: {}",
        String::from_utf8_lossy(&list_output.stderr)
    );

    let list_output_str = String::from_utf8_lossy(&list_output.stdout);
    assert!(
        list_output_str.contains("testuser/test-project"),
        "Docker image not found in list output"
    );
}

#[test]
#[cfg_attr(not(feature = "docker_tests"), ignore)]
fn test_container_runtime() {
    if is_ci() {
        println!("Skipping Docker tests in CI environment");
        return;
    }
    let (_temp_dir, project_dir) = setup_test_project();

    // Build the container
    let build_output = StdCommand::new("make")
        .arg("build")
        .current_dir(&project_dir)
        .output()
        .unwrap_or_else(|e| panic!("Failed to build container: {}", e));

    assert!(
        build_output.status.success(),
        "Make build failed: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    // Test container environment
    let container_test = StdCommand::new("docker")
        .args(["run", "--rm", "testuser/test-project:latest", "env"])
        .current_dir(&project_dir)
        .output()
        .unwrap_or_else(|e| panic!("Failed to run container: {}", e));

    assert!(container_test.status.success(), "Container failed to run");
    let output = String::from_utf8_lossy(&container_test.stdout);
    assert!(output.contains("PATH=/usr/local/bin"));

    // Test user configuration
    let entrypoint_test = StdCommand::new("docker")
        .args([
            "run",
            "--rm",
            "testuser/test-project:latest",
            "sh",
            "-c",
            "id",
        ])
        .current_dir(&project_dir)
        .output()
        .unwrap_or_else(|e| panic!("Failed to test entrypoint: {}", e));

    let output = String::from_utf8_lossy(&entrypoint_test.stdout);
    assert!(
        output.contains("uid=1000"),
        "Container should run as uid 1000"
    );
    assert!(
        output.contains("essex"),
        "Container should run as essex user"
    );

    // Clean up
    StdCommand::new("docker")
        .args(["rmi", "-f", "testuser/test-project:latest"])
        .output()
        .expect("Failed to remove docker image");
}

#[test]
fn test_template_variable_substitution() {
    let (_temp_dir, project_dir) = setup_test_project();

    // Check Makefile variables
    let makefile_path = project_dir.join("Makefile");
    let makefile_content = fs::read_to_string(&makefile_path)
        .unwrap_or_else(|e| panic!("Failed to read Makefile at {:?}: {}", makefile_path, e));

    assert!(makefile_content.contains("REPO_NAMESPACE        ?= testuser"));
    assert!(makefile_content.contains("REPO_USERNAME         ?= testuser"));
    assert!(makefile_content.contains("IMAGE_NAME            ?= test-project"));

    // Check Dockerfile labels
    let dockerfile_path = project_dir.join("Dockerfile");
    let dockerfile_content = fs::read_to_string(&dockerfile_path)
        .unwrap_or_else(|e| panic!("Failed to read Dockerfile at {:?}: {}", dockerfile_path, e));

    assert!(dockerfile_content
        .contains("org.opencontainers.image.source=\"https://github.com/testuser/test-project\""));
    assert!(dockerfile_content.contains("org.opencontainers.image.vendor=\"Test Company\""));
}
