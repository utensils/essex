use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, Debug, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List available templates
    List,

    /// Create a new project from a template
    New {
        /// Template to use
        #[arg(value_parser = validate_template)]
        template: String,

        /// Project path in format: username/project-name
        project: String,

        /// Docker Hub username (optional)
        #[arg(short, long)]
        username: Option<String>,

        /// Vendor name for labels
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

// Function to validate template existence
fn validate_template(s: &str) -> Result<String, String> {
    // This will be replaced with actual template validation in main.rs
    Ok(s.to_string())
}