use clap::Parser;
use std::path;
/// Search for a pattern in a file and display the lines that contai it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: path::PathBuf,
}
fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path {:?}", args.pattern, args.path);
}
