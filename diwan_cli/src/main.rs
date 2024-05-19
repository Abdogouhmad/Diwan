// use anyhow::Context;
// use std::fs::File;
// use std::io::BufReader
use clap::Parser;
use diwan_core::cursor::Editor;
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

    // If a file path is provided, open the editor with that file
    if let Some(path) = args.file {
        // Open the file
        // let file = File::open(&path).context("failed to open file")?;
        // let _ = BufReader::new(file);

        // TODO: open editor with the file
        println!("{}", path.display());
    } else {
        // TODO: run editor without any file
        Editor.run();
    }

    // init editor
    //let mut editor = diwan::Editor::new();

    // run editor
    //editor.run();

    Ok(())
}
