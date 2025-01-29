#[cfg(test)]
mod tests {
    use super::*;

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
}