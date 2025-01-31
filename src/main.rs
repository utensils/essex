use clap::Parser;
use cli::Cli;
use error::Result;

mod cli;
mod error;
mod template;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.execute()
}

#[cfg(test)]
mod tests {
    use include_dir::{include_dir, Dir};

    #[test]
    fn test_template_embedding() {
        static TEMPLATES: Dir = include_dir!("templates");
        assert!(TEMPLATES.get_file("basic/Dockerfile").is_some());
        assert!(TEMPLATES.get_file("basic/Makefile").is_some());
        assert!(TEMPLATES.get_file("basic/README.md").is_some());
    }
}
