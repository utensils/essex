use clap::{Parser, Subcommand};

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
}
