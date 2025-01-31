use proptest::prelude::*;
use std::path::PathBuf;
use tempfile::tempdir;

// Import from the crate root
use essex::template::{TemplateContext, TemplateEngine};

proptest! {
    #[test]
    fn test_template_context_creation(
        namespace in "[a-zA-Z0-9_-]{1,25}",
        name in "[a-zA-Z0-9_-]{1,25}",
        username in "[a-zA-Z0-9_-]{1,50}",
        vendor in "[a-zA-Z0-9_-]{1,50}"
    ) {
        let project = format!("{}/{}", namespace, name);
        let context = TemplateContext::new(
            &project,
            Some(username.clone()),
            Some(vendor.clone())
        ).unwrap();

        let context_map = context.into_context();

        // Verify context properties
        prop_assert_eq!(context_map.get("repo_namespace").unwrap().as_str().unwrap(), namespace);
        prop_assert_eq!(context_map.get("image_name").unwrap().as_str().unwrap(), name);
        prop_assert_eq!(context_map.get("repo_username").unwrap().as_str().unwrap(), username);
        prop_assert_eq!(context_map.get("vendor").unwrap().as_str().unwrap(), vendor);
    }

    #[test]
    fn test_template_generation_with_various_inputs(
        namespace in "[a-zA-Z0-9_-]{1,25}",
        name in "[a-zA-Z0-9_-]{1,25}",
        username in "[a-zA-Z0-9_-]{1,50}",
        vendor in "[a-zA-Z0-9_-]{1,50}"
    ) {
        let project = format!("{}/{}", namespace, name);
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("test-output");

        let mut engine = TemplateEngine::new(".").unwrap();
        let context = TemplateContext::new(
            &project,
            Some(username),
            Some(vendor)
        ).unwrap();

        let result = engine.generate("basic", context, &output_dir);
        prop_assert!(result.is_ok());

        // Verify essential files exist
        let required_files = vec![
            "Dockerfile",
            "Makefile",
            "README.md",
            "runtime-assets/usr/local/bin/entrypoint.sh",
        ];

        for file in required_files {
            prop_assert!(output_dir.join(file).exists());
        }
    }
}

#[test]
fn test_template_context_edge_cases() {
    // Test empty project name
    assert!(TemplateContext::new("", None, None).is_err());

    // Test very long inputs
    let long_string = "a".repeat(256);
    assert!(TemplateContext::new(&long_string, None, None).is_err());

    // Test special characters
    assert!(TemplateContext::new("project#name", None, None).is_err());

    // Test missing namespace separator
    assert!(TemplateContext::new("projectname", None, None).is_err());

    // Test empty namespace
    assert!(TemplateContext::new("/projectname", None, None).is_err());

    // Test empty name
    assert!(TemplateContext::new("namespace/", None, None).is_err());

    // Test multiple separators
    assert!(TemplateContext::new("namespace/project/name", None, None).is_err());
}

#[test]
fn test_template_generation_error_cases() {
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("test-output");

    let mut engine = TemplateEngine::new(".").unwrap();

    // Test with non-existent template
    let context = TemplateContext::new("test/project", None, None).unwrap();
    assert!(engine
        .generate("non_existent_template", context, &output_dir)
        .is_err());

    // Test with invalid output directory
    let context = TemplateContext::new("test/project", None, None).unwrap();
    let invalid_dir = PathBuf::from("/nonexistent/directory");
    assert!(engine.generate("basic", context, &invalid_dir).is_err());
}
