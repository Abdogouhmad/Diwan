use anyhow::Context;
use clap::Parser;
use diwan_core::editor::Editor;
use std::{path::PathBuf, process::exit};

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

    // Create a new editor instance
    let mut editor = Editor::new()?;

    // If a file path is provided, open the editor with that file
    if let Some(path) = args.file {
        // TODO: open editor with the file
        println!("{}", path.display());
    } else {
        editor.run().context("Failed to load Diwan editor")?;
        exit(1);
    }

    Ok(())
}
