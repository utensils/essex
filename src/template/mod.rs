use chrono::Utc;
use include_dir::{include_dir, Dir};
use serde::Serialize;
use std::path::Path;
use tera::{Context, Tera};

use crate::error::{EssexError, Result};

// Embed templates at compile time
static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

#[derive(Debug, Serialize)]
pub struct TemplateContext {
    pub repo_namespace: String,
    pub repo_username: String,
    pub image_name: String,
    pub vendor: String,
    pub build_date: String,
    pub version: String,
    pub vcs_ref: String,
}

impl TemplateContext {
    pub fn new(project: &str, username: Option<String>, vendor: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = project.split('/').collect();
        if parts.len() != 2 {
            return Err(EssexError::InvalidProjectName(project.to_string()));
        }

        Ok(Self {
            repo_namespace: parts[0].to_string(),
            repo_username: username.unwrap_or_else(|| parts[0].to_string()),
            image_name: parts[1].to_string(),
            vendor: vendor.unwrap_or_else(|| "Unknown".to_string()),
            build_date: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            version: "0.1.0".to_string(),
            vcs_ref: "initial".to_string(),
        })
    }

    pub fn into_context(self) -> Context {
        let mut context = Context::new();
        context.insert("repo_namespace", &self.repo_namespace);
        context.insert("repo_username", &self.repo_username);
        context.insert("image_name", &self.image_name);
        context.insert("vendor", &self.vendor);
        context.insert("build_date", &self.build_date);
        context.insert("version", &self.version);
        context.insert("vcs_ref", &self.vcs_ref);
        context
    }
}

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new<P: AsRef<Path>>(_templates_dir: P) -> Result<Self> {
        // Create an empty Tera instance since we'll render strings directly
        let tera = Tera::default();
        Ok(Self { tera })
    }

    pub fn list_templates(&self) -> Result<Vec<String>> {
        let mut templates = Vec::new();
        for entry in TEMPLATES.entries() {
            if entry.as_dir().is_some() {
                templates.push(entry.path().to_string_lossy().into_owned());
            }
        }
        Ok(templates)
    }

    pub fn generate(
        &mut self,
        template: &str,
        context: TemplateContext,
        output_dir: &Path,
    ) -> Result<()> {
        let template_dir = TEMPLATES
            .get_dir(template)
            .ok_or_else(|| EssexError::TemplateNotFound(template.to_string()))?;

        // Create the output directory
        std::fs::create_dir_all(output_dir)?;

        self.copy_template_files(template_dir, output_dir, &context.into_context())?;
        Ok(())
    }

    fn copy_template_files(&mut self, dir: &Dir, to: &Path, context: &Context) -> Result<()> {
        for entry in dir.entries() {
            // Get the relative path from the template root
            let rel_path = entry
                .path()
                .strip_prefix(dir.path())
                .unwrap_or(entry.path());
            let target = to.join(rel_path);

            if let Some(subdir) = entry.as_dir() {
                std::fs::create_dir_all(&target)?;
                self.copy_template_files(subdir, &target, context)?;
            } else if let Some(file) = entry.as_file() {
                // Create parent directories if they don't exist
                if let Some(parent) = target.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                let content = file.contents_utf8().ok_or_else(|| {
                    EssexError::TemplateNotFound(entry.path().to_string_lossy().to_string())
                })?;
                let rendered = self.tera.render_str(content, context)?;
                std::fs::write(&target, rendered)?;

                // Set executable permissions for shell scripts
                #[cfg(unix)]
                if entry.path().extension().map_or(false, |ext| ext == "sh") {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = std::fs::metadata(&target)?.permissions();
                    perms.set_mode(0o755);
                    std::fs::set_permissions(&target, perms)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_template_engine() -> Result<()> {
        // Create a temporary directory for output
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("test-project");

        // Initialize template engine
        let mut engine = TemplateEngine::new("ignored")?;

        // List available templates
        let templates = engine.list_templates()?;
        assert!(!templates.is_empty(), "Should find at least one template");
        assert!(
            templates.contains(&"basic".to_string()),
            "Should find basic template"
        );

        // Create a test context
        let context = TemplateContext::new("test/project", None, None)?;

        // Generate project from basic template
        engine.generate("basic", context, &output_dir)?;

        // Verify that files were created
        assert!(output_dir.exists(), "Output directory should exist");

        Ok(())
    }

    #[test]
    fn test_template_not_found() {
        let mut engine = TemplateEngine::new("ignored").unwrap();
        let context = TemplateContext::new("test/project", None, None).unwrap();
        let temp_dir = tempdir().unwrap();

        let result = engine.generate("non-existent", context, temp_dir.path());
        assert!(result.is_err(), "Should error on non-existent template");

        if let Err(e) = result {
            assert!(matches!(e, EssexError::TemplateNotFound(_)));
        }
    }
}
