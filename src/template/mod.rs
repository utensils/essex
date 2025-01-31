use chrono::Utc;
use include_dir::{include_dir, Dir, DirEntry};
use serde::Serialize;
use std::path::Path;
use tera::{Context, Tera};
use tokio::fs;
use tokio::task;
use tokio::task::JoinError;
use std::future::Future;
use std::pin::Pin;

use crate::error::{Error, Result};

static TEMPLATES: Dir = include_dir!("templates");

#[derive(Debug, Clone, Serialize)]
pub struct TemplateContext {
    pub repo_username: String,
    pub repo_namespace: String,
    pub image_name: String,
    pub vendor: String,
    pub version: String,
    pub build_date: String,
    pub vcs_ref: String,
}

impl TemplateContext {
    pub fn new(project: &str, username: Option<String>, vendor: Option<String>) -> Result<Self> {
        let parts: Vec<&str> = project.split('/').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(Error::InvalidProjectName(project.to_string()));
        }

        // Validate that namespace and project name only contain alphanumeric characters, hyphens, and underscores
        let valid_chars = |s: &str| {
            !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        };

        if !valid_chars(parts[0]) || !valid_chars(parts[1]) {
            return Err(Error::InvalidProjectName(project.to_string()));
        }

        Ok(Self {
            repo_username: username.unwrap_or_else(|| "example".to_string()),
            repo_namespace: parts[0].to_string(),
            image_name: parts[1].to_string(),
            vendor: vendor.unwrap_or_else(|| "Example Corp".to_string()),
            version: "0.1.0".to_string(),
            build_date: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            vcs_ref: "HEAD".to_string(),
        })
    }

    pub fn into_context(self) -> Context {
        let mut context = Context::new();
        context.insert("repo_username", &self.repo_username);
        context.insert("repo_namespace", &self.repo_namespace);
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

    /// Generate a project asynchronously
    pub async fn generate_async(
        &mut self,
        template: &str,
        context: TemplateContext,
        output_dir: &Path,
    ) -> Result<()> {
        // Validate template exists
        if !self.list_templates()?.contains(&template.to_string()) {
            return Err(Error::TemplateNotFound(template.to_string()));
        }

        // Create output directory
        fs::create_dir_all(output_dir).await?;

        // Convert context to Tera context
        let context = context.into_context();

        // Find the template directory
        let template_dir = TEMPLATES
            .get_dir(template)
            .ok_or_else(|| Error::TemplateNotFound(template.to_string()))?;

        // Process template files recursively
        self.process_directory_async(template_dir, output_dir, &context).await?;

        Ok(())
    }

    /// Process a directory recursively
    fn process_directory_async<'a>(
        &'a self,
        dir: &'a Dir<'_>,
        output_dir: &'a Path,
        context: &'a Context,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut tasks = Vec::new();

            for entry in dir.entries() {
                match entry {
                    DirEntry::Dir(subdir) => {
                        let new_output_dir = output_dir.join(subdir.path().strip_prefix(dir.path())?);
                        fs::create_dir_all(&new_output_dir).await?;
                        self.process_directory_async(subdir, &new_output_dir, context).await?;
                    }
                    DirEntry::File(file) => {
                        let output_path = output_dir.join(file.path().strip_prefix(dir.path())?);
                        let content = file.contents_utf8()
                            .ok_or_else(|| Error::InvalidTemplate("Template file is not valid UTF-8".to_string()))?
                            .to_string();
                        let context = context.clone();
                        let output_path = output_path.to_path_buf();

                        tasks.push(task::spawn(async move {
                            if let Some(parent) = output_path.parent() {
                                fs::create_dir_all(parent).await?;
                            }

                            let rendered = tera::Tera::one_off(&content, &context, false)?;
                            fs::write(&output_path, rendered).await?;

                            // Set executable permissions for .sh files
                            if output_path.extension().map_or(false, |ext| ext == "sh") {
                                #[cfg(unix)]
                                {
                                    use std::os::unix::fs::PermissionsExt;
                                    let mut perms = fs::metadata(&output_path).await?.permissions();
                                    perms.set_mode(0o755);
                                    fs::set_permissions(&output_path, perms).await?;
                                }
                            }
                            Ok::<(), anyhow::Error>(())
                        }));
                    }
                }
            }

            // Wait for all tasks to complete
            for task in tasks {
                task.await.map_err(|e: JoinError| Error::TaskJoinError(e.to_string()))??;
            }

            Ok(())
        })
    }

    pub fn generate(
        &mut self,
        template: &str,
        context: TemplateContext,
        output_dir: &Path,
    ) -> Result<()> {
        // Validate template exists
        if !self.list_templates()?.contains(&template.to_string()) {
            return Err(Error::TemplateNotFound(template.to_string()));
        }

        // Create output directory
        std::fs::create_dir_all(output_dir)?;

        // Convert context to Tera context
        let context = context.into_context();

        // Find the template directory
        let template_dir = TEMPLATES
            .get_dir(template)
            .ok_or_else(|| Error::TemplateNotFound(template.to_string()))?;

        // Copy and process template files
        self.copy_template_files(template_dir, output_dir, &context)?;

        Ok(())
    }

    fn copy_template_files(&mut self, dir: &Dir, to: &Path, context: &Context) -> Result<()> {
        // Process all entries in the directory
        for entry in dir.entries() {
            match entry {
                DirEntry::Dir(subdir) => {
                    // Create the subdirectory
                    let rel_path = subdir.path().strip_prefix(dir.path())?;
                    let output_subdir = to.join(rel_path);
                    std::fs::create_dir_all(&output_subdir)?;
                    
                    // Recursively process the subdirectory
                    self.copy_template_files(subdir, &output_subdir, context)?;
                }
                DirEntry::File(file) => {
                    let rel_path = file.path().strip_prefix(dir.path())?;
                    let output_path = to.join(rel_path);

                    if let Some(parent) = output_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }

                    let content = file
                        .contents_utf8()
                        .ok_or_else(|| Error::InvalidTemplate("Template file is not valid UTF-8".to_string()))?;
                    let rendered = self.tera.render_str(content, context)?;
                    std::fs::write(&output_path, rendered)?;

                    // Set executable permissions for .sh files
                    if output_path.extension().map_or(false, |ext| ext == "sh") {
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            let mut perms = std::fs::metadata(&output_path)?.permissions();
                            perms.set_mode(0o755);
                            std::fs::set_permissions(&output_path, perms)?;
                        }
                    }
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
        let temp_dir = tempdir()?;
        let mut engine = TemplateEngine::new(".")?;
        let context = TemplateContext::new(
            "test/project",
            Some("testuser".to_string()),
            Some("Test Company".to_string()),
        )?;

        engine.generate("basic", context, temp_dir.path())?;

        // Verify essential files exist
        let dockerfile = temp_dir.path().join("Dockerfile");
        assert!(dockerfile.exists());
        let content = std::fs::read_to_string(dockerfile)?;
        assert!(content.contains("org.opencontainers.image.authors=\"testuser <contact@example.com>\""));

        Ok(())
    }

    #[test]
    fn test_template_not_found() {
        let engine = TemplateEngine::new(".").unwrap();
        let templates = engine.list_templates().unwrap();
        assert!(!templates.contains(&"non_existent".to_string()));
    }
}
