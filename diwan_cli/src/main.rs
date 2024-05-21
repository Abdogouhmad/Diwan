use anyhow::Context;
use diwan_core::editor::Editor;
use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // If there is at least one argument after the program name
    if args.len() > 1 {
        // Assume the first argument after the program name is the file path
        let file_path = PathBuf::from(&args[1]);
        println!("Opening file: {}", file_path.display());
        // Handle the file path here
    } else {
        // Create a new editor instance
        let mut editor = Editor::new().context("Failed to create instance of editor")?;

        // Run the editor
        editor.run().context("Failed to load Diwan editor")?;

        // Drop the editor to restore terminal state
        drop(editor);
    }

    Ok(())
}
