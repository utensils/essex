use clap::{CommandFactory, Parser};
use clap_complete::{
    generate,
    shells::{Bash, Zsh},
};
use std::path::PathBuf;

use crate::error::{Error, Result};
use crate::template::{TemplateContext, TemplateEngine};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// List available templates
    List,

    /// Create a new project from a template
    New {
        /// Template to use
        template: String,

        /// Project name in the format namespace/project
        project: String,

        /// Username for the project (optional)
        #[arg(short, long)]
        username: Option<String>,

        /// Vendor name for the project (optional)
        #[arg(short, long)]
        vendor: Option<String>,
    },

    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,

        /// Output directory for completion script
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum Shell {
    Bash,
    Zsh,
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        let mut engine = TemplateEngine::new(".")?;

        match self.command {
            Commands::List => {
                let templates = engine.list_templates()?;
                println!("Available templates:");
                for template in templates {
                    println!("  - {}", template);
                }
                Ok(())
            }
            Commands::New {
                template,
                project,
                username,
                vendor,
            } => {
                // Validate template exists
                let templates = engine.list_templates()?;
                if !templates.contains(&template) {
                    return Err(Error::TemplateNotFound(template));
                }

                let context = TemplateContext::new(&project, username, vendor)?;
                let parts: Vec<&str> = project.split('/').collect();
                if parts.len() != 2 {
                    return Err(Error::InvalidProjectName(project));
                }

                // Create project directory inside a directory named after the namespace
                let namespace_dir = PathBuf::from(parts[0]);
                let project_dir = namespace_dir.join(parts[1]);

                if project_dir.exists() {
                    return Err(Error::InvalidTemplate(format!(
                        "Directory '{}' already exists",
                        project_dir.display()
                    )));
                }

                println!(
                    "Creating new project '{}' using template '{}'",
                    project, template
                );
                engine.generate(&template, context, &project_dir)?;
                println!("Project created successfully!");
                Ok(())
            }
            Commands::Completion { shell, output } => {
                let mut cmd = Cli::command();
                let bin_name = cmd.get_name().to_string();

                match shell {
                    Shell::Bash => {
                        if let Some(out_dir) = output {
                            std::fs::create_dir_all(&out_dir)?;
                            let mut file =
                                std::fs::File::create(out_dir.join(format!("{}.bash", bin_name)))?;
                            generate(Bash, &mut cmd, bin_name, &mut file);
                            println!("Bash completion script written to {:?}", file);
                        } else {
                            generate(Bash, &mut cmd, bin_name, &mut std::io::stdout());
                        }
                    }
                    Shell::Zsh => {
                        if let Some(out_dir) = output {
                            std::fs::create_dir_all(&out_dir)?;
                            let mut file =
                                std::fs::File::create(out_dir.join(format!("_{}", bin_name)))?;
                            generate(Zsh, &mut cmd, bin_name, &mut file);
                            println!("Zsh completion script written to {:?}", file);
                        } else {
                            generate(Zsh, &mut cmd, bin_name, &mut std::io::stdout());
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_command_parsing() {
        let cli = Cli::try_parse_from(["essex", "list"]).unwrap();
        assert!(matches!(cli.command, Commands::List));
    }

    #[test]
    fn test_new_command_parsing() {
        let cli = Cli::try_parse_from([
            "essex",
            "new",
            "basic",
            "test/project",
            "--username",
            "testuser",
            "--vendor",
            "Test Corp",
        ])
        .unwrap();

        match cli.command {
            Commands::New {
                template,
                project,
                username,
                vendor,
            } => {
                assert_eq!(template, "basic");
                assert_eq!(project, "test/project");
                assert_eq!(username, Some("testuser".to_string()));
                assert_eq!(vendor, Some("Test Corp".to_string()));
            }
            _ => panic!("Expected New command"),
        }
    }

    #[test]
    fn test_completion_command_parsing() {
        let cli = Cli::try_parse_from(["essex", "completion", "bash"]).unwrap();
        match cli.command {
            Commands::Completion { shell, output } => {
                assert!(matches!(shell, Shell::Bash));
                assert!(output.is_none());
            }
            _ => panic!("Expected Completion command"),
        }
    }

    #[test]
    fn test_validate_template() {
        let cli = Cli::try_parse_from(["essex", "new", "basic", "test/project"]).unwrap();
        match cli.command {
            Commands::New {
                template, project, ..
            } => {
                assert_eq!(template, "basic");
                assert_eq!(project, "test/project");
            }
            _ => panic!("Expected New command"),
        }
    }
}
