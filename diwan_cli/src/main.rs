use anyhow::Context;
use diwan_core::buffer::Buffer;
use diwan_core::editor::Editor;
use std::env;

fn main() -> anyhow::Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Match on the command-line arguments to handle different cases
    match args.get(1).map(String::as_str) {
        Some("-h") | Some("--help") => {
            print_help();
            Ok(())
        }
        _ => {
            // Determine if a file path was provided
            let file_path = args.get(1).cloned();

            // Create the buffer and editor
            let buffer = Buffer::from_file(file_path);
            let mut editor = Editor::new(buffer).context("Failed to create instance of editor")?;

            // Run the editor
            editor.run().context("Failed to load Diwan editor")?;

            // Drop the editor to restore terminal state
            drop(editor);
            Ok(())
        }
    }
}

fn print_help() {
    println!("Diwan Editor Help");
    println!("Usage: diwan [OPTIONS] [FILE]");
    println!();
    println!("Arguments:");
    println!("  FILE             The file to open in the editor");
    println!();
    println!("Options:");
    println!("  -h, --help       Print this help message");
    println!();
}
