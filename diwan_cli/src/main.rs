use anyhow::Context;
use clap::Parser;
use diwan_core::editor::Editor;
use std::path::PathBuf;

/// diwan is a simple Rust text editor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    // Parse the arguments
    let args = Args::parse();

    // If a file path is provided, print the file path
    if let Some(path) = args.file {
        println!(" hey {}", path.display());
    } else {
        // Create a new editor instance
        let mut editor = Editor::new().context("Failed to return instance of editor")?;
        // Run the editor
        editor.run().context("Failed to load Diwan editor")?;
        // Drop the editor to restore terminal state
        drop(editor);
    }

    Ok(())
}
