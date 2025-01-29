use std::path::{Path, PathBuf};
use chrono::Utc;
use serde::Serialize;
use tera::{Tera, Context};

use crate::error::{Result, EssexError};

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
            version: "0.1.0".to_string(), // This could be fetched from git
            vcs_ref: "initial".to_string(), // This could be fetched from git
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
    templates_dir: PathBuf,
    tera: Tera,
}

impl TemplateEngine {
    pub fn new<P: AsRef<Path>>(templates_dir: P) -> Result<Self> {
        let templates_dir = templates_dir.as_ref().to_path_buf();
        let tera = Tera::new(&templates_dir.join("**/*").to_string_lossy())?;
        
        Ok(Self {
            templates_dir,
            tera,
        })
    }

    pub fn list_templates(&self) -> Result<Vec<String>> {
        let mut templates = Vec::new();
        for entry in std::fs::read_dir(&self.templates_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                templates.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
        Ok(templates)
    }

    pub fn generate(&mut self, template: &str, context: TemplateContext, output_dir: &Path) -> Result<()> {
        let template_dir = self.templates_dir.join(template);
        if !template_dir.exists() {
            return Err(EssexError::TemplateNotFound(template.to_string()));
        }

        self.copy_template_files(&template_dir, output_dir, &context.into_context())?;
        Ok(())
    }

    fn copy_template_files(&mut self, from: &Path, to: &Path, context: &Context) -> Result<()> {
        std::fs::create_dir_all(to)?;

        for entry in std::fs::read_dir(from)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(from).unwrap();
            let target = to.join(relative);

            if path.is_dir() {
                std::fs::create_dir_all(&target)?;
                self.copy_template_files(&path, &target, context)?;
            } else {
                let content = std::fs::read_to_string(&path)?;
                let rendered = self.tera.render_str(&content, context)?;
                std::fs::write(target, rendered)?;
            }
        }

        Ok(())
    }
}