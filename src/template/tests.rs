#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_template_context_creation() {
        let context = TemplateContext::new(
            "user/project",
            Some("dockeruser".to_string()),
            Some("MyCompany".to_string())
        ).unwrap();

        assert_eq!(context.repo_namespace, "user");
        assert_eq!(context.repo_username, "dockeruser");
        assert_eq!(context.image_name, "project");
        assert_eq!(context.vendor, "MyCompany");
    }

    #[test]
    fn test_template_context_defaults() {
        let context = TemplateContext::new(
            "user/project",
            None,
            None
        ).unwrap();

        assert_eq!(context.repo_namespace, "user");
        assert_eq!(context.repo_username, "user");
        assert_eq!(context.image_name, "project");
        assert_eq!(context.vendor, "Unknown");
    }

    #[test]
    fn test_invalid_project_name() {
        let result = TemplateContext::new("invalid", None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_template_context_validation() -> Result<()> {
        // Test valid project name
        let ctx = TemplateContext::new("namespace/project", None, None)?;
        assert_eq!(ctx.repo_namespace, "namespace");
        assert_eq!(ctx.image_name, "project");
        assert_eq!(ctx.repo_username, "example");
        assert_eq!(ctx.vendor, "Example Corp");

        // Test custom username and vendor
        let ctx = TemplateContext::new(
            "custom/project",
            Some("user123".to_string()),
            Some("Custom Inc".to_string()),
        )?;
        assert_eq!(ctx.repo_username, "user123");
        assert_eq!(ctx.vendor, "Custom Inc");

        // Test invalid project names
        assert!(TemplateContext::new("invalid", None, None).is_err());
        assert!(TemplateContext::new("/project", None, None).is_err());
        assert!(TemplateContext::new("namespace/", None, None).is_err());
        assert!(TemplateContext::new("name space/project", None, None).is_err());
        assert!(TemplateContext::new("namespace/pro ject", None, None).is_err());
        assert!(TemplateContext::new("namespace/project!", None, None).is_err());

        Ok(())
    }

    #[test]
    fn test_template_context_into_context() -> Result<()> {
        let ctx = TemplateContext::new("namespace/project", None, None)?;
        let tera_ctx = ctx.clone().into_context();

        // Verify all fields are properly inserted into the Tera context
        assert_eq!(
            tera_ctx.get("repo_username").unwrap().as_str().unwrap(),
            ctx.repo_username
        );
        assert_eq!(
            tera_ctx.get("repo_namespace").unwrap().as_str().unwrap(),
            ctx.repo_namespace
        );
        assert_eq!(
            tera_ctx.get("image_name").unwrap().as_str().unwrap(),
            ctx.image_name
        );
        assert_eq!(tera_ctx.get("vendor").unwrap().as_str().unwrap(), ctx.vendor);
        assert_eq!(
            tera_ctx.get("version").unwrap().as_str().unwrap(),
            ctx.version
        );
        assert_eq!(
            tera_ctx.get("build_date").unwrap().as_str().unwrap(),
            ctx.build_date
        );
        assert_eq!(
            tera_ctx.get("vcs_ref").unwrap().as_str().unwrap(),
            ctx.vcs_ref
        );

        Ok(())
    }

    #[test]
    fn test_template_engine_empty_dir() -> Result<()> {
        let temp_dir = tempdir()?;
        let engine = TemplateEngine::new(temp_dir.path())?;
        let templates = engine.list_templates()?;
        assert!(!templates.is_empty(), "Should list built-in templates");
        Ok(())
    }

    #[test]
    fn test_template_engine_invalid_template() -> Result<()> {
        let temp_dir = tempdir()?;
        let engine = TemplateEngine::new(temp_dir.path())?;
        let ctx = TemplateContext::new("test/project", None, None)?;
        let result = engine.generate("non-existent", ctx, &temp_dir.path());
        assert!(result.is_err());
        if let Err(Error::TemplateNotFound(name)) = result {
            assert_eq!(name, "non-existent");
        } else {
            panic!("Expected TemplateNotFound error");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_template_engine_async_generation() -> Result<()> {
        let temp_dir = tempdir()?;
        let engine = TemplateEngine::new(temp_dir.path())?;
        let ctx = TemplateContext::new("test/project", None, None)?;
        
        // Test with non-existent template
        let result = engine
            .generate_async("non-existent", ctx.clone(), temp_dir.path())
            .await;
        assert!(result.is_err());
        
        // Test with valid template
        let templates = engine.list_templates()?;
        if let Some(template) = templates.first() {
            let result = engine
                .generate_async(template, ctx, temp_dir.path())
                .await;
            assert!(result.is_ok());
        }
        
        Ok(())
    }
}