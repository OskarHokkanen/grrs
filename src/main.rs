use anyhow::{Context, Result};
use clap::Parser;
/// Search for a pattern in a file and display the lines that contai it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;
    for line in content.lines() {
        // TODO: Read about BufReader to not read the whole file (read_to_string()) into memory.
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
