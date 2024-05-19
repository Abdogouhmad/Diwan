// use anyhow::Context;
// use std::fs::File;
// use std::io::BufReader
use clap::Parser;
use diwan_core::core::Core;
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
    let editor = Core;

    // If a file path is provided, open the editor with that file
    if let Some(path) = args.file {
        // TODO: open editor with the file
        println!("{}", path.display());
    } else {
        let logs = editor.run();
        if logs.is_err() {
            eprintln!("can't open the Editor");
            exit(1);
        }
    }

    Ok(())
}
