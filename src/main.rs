mod cli;
mod error;
mod template;

use std::path::PathBuf;
use clap::Parser;
use cli::{Cli, Commands};
use error::{Result, EssexError};
use template::{TemplateEngine, TemplateContext};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let templates_dir = PathBuf::from("templates");
    let mut engine = TemplateEngine::new(templates_dir)?;

    match cli.command {
        Commands::List => {
            let templates = engine.list_templates()?;
            println!("Available templates:");
            for template in templates {
                println!("  - {}", template);
            }
        }
        Commands::New { template, project, username, vendor } => {
            let context = TemplateContext::new(&project, username, vendor)?;
            let output_dir = PathBuf::from(&project);
            
            if output_dir.exists() {
                return Err(EssexError::ProjectDirectoryError(
                    format!("Directory '{}' already exists", project)
                ));
            }

            println!("Creating new project '{}' using template '{}'", project, template);
            engine.generate(&template, context, &output_dir)?;
            println!("Project created successfully!");
        }
    }

    Ok(())
}