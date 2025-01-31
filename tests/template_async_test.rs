use tempfile::tempdir;
use tokio::runtime::Runtime;

use essex::template::{TemplateContext, TemplateEngine};

#[test]
fn test_async_template_generation() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("test-output");

        let mut engine = TemplateEngine::new(".").unwrap();
        let context = TemplateContext::new(
            "test/project",
            Some("testuser".to_string()),
            Some("Test Corp".to_string()),
        )
        .unwrap();

        let result = engine.generate_async("basic", context, &output_dir).await;
        assert!(result.is_ok());

        // Verify essential files exist
        let required_files = vec![
            "Dockerfile",
            "Makefile",
            "README.md",
            "runtime-assets/usr/local/bin/entrypoint.sh",
        ];

        for file in required_files {
            let file_path = output_dir.join(file);
            assert!(
                file_path.exists(),
                "File does not exist: {}",
                file_path.display()
            );
        }

        let makefile = std::fs::read_to_string(output_dir.join("Makefile")).unwrap();
        assert!(makefile.contains("REPO_USERNAME         ?= testuser"));
    });
}

#[test]
fn test_async_template_generation_parallel() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let temp_dir = tempdir().unwrap();
        let base_dir = temp_dir.path();

        let mut handles = Vec::new();

        for i in 0..5 {
            let output_dir = base_dir.join(format!("test-output-{}", i));
            let context = TemplateContext::new(
                &format!("test/project-{}", i),
                Some("testuser".to_string()),
                Some("Test Corp".to_string()),
            )
            .unwrap();

            let mut engine = TemplateEngine::new(".").unwrap();
            handles.push(tokio::spawn(async move {
                engine
                    .generate_async("basic", context, &output_dir)
                    .await
                    .unwrap();
            }));
        }

        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify all projects were created
        for i in 0..5 {
            let output_dir = base_dir.join(format!("test-output-{}", i));
            let required_files = vec![
                "Dockerfile",
                "Makefile",
                "README.md",
                "runtime-assets/usr/local/bin/entrypoint.sh",
            ];

            for file in required_files {
                let file_path = output_dir.join(file);
                assert!(
                    file_path.exists(),
                    "File does not exist in project {}: {}",
                    i,
                    file_path.display()
                );
            }
        }
    });
}

#[test]
fn test_async_template_error_handling() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("test-output");

        let mut engine = TemplateEngine::new(".").unwrap();

        // Test with non-existent template
        let context = TemplateContext::new(
            "test/project",
            Some("testuser".to_string()),
            Some("Test Corp".to_string()),
        )
        .unwrap();

        let result = engine
            .generate_async("non_existent_template", context, &output_dir)
            .await;
        assert!(result.is_err());
    });
}

#[test]
fn test_async_template_variable_substitution() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("test-output");

        let mut engine = TemplateEngine::new(".").unwrap();
        let context = TemplateContext::new(
            "myorg/myproject",
            Some("testuser".to_string()),
            Some("Test Corp".to_string()),
        )
        .unwrap();

        let result = engine.generate_async("basic", context, &output_dir).await;
        assert!(result.is_ok());

        // Verify variable substitution in Dockerfile
        let dockerfile = std::fs::read_to_string(output_dir.join("Dockerfile")).unwrap();
        assert!(dockerfile
            .contains("org.opencontainers.image.authors=\"testuser <contact@example.com>\""));
        assert!(dockerfile.contains("org.opencontainers.image.vendor=\"Test Corp\""));

        // Verify variable substitution in Makefile
        let makefile = std::fs::read_to_string(output_dir.join("Makefile")).unwrap();
        assert!(makefile.contains("REPO_USERNAME         ?= testuser"));
        assert!(makefile.contains("REPO_NAMESPACE        ?= myorg"));
        assert!(makefile.contains("IMAGE_NAME            ?= myproject"));
    });
}
