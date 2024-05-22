use std::env;

fn main() -> anyhow::Result<()> {
    // collect the arguments provided
    let args: Vec<String> = env::args().collect();
    // see if the argument contains -h
    match args.get(1).map(String::as_str) {
        Some("-h") | Some("--help") => {
            print_help();
        }
        _ => {
            println!("run editor")
        }
    }
    Ok(())
}
fn print_help() {
    println!("Diwan is a minimal rust text editor like vim :)");
    println!("Usage: diwan [OPTIONS] [FILE]");
    println!();
    println!("Arguments:");
    println!("  FILE             The file to open in the editor");
    println!();
    println!("Options:");
    println!("  -h, --help       Print this help message");
    println!();
}
