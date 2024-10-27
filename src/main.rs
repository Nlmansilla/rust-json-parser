mod errors;
mod lexer;
mod tokens;
mod tests;

use anyhow::{Context, Result, anyhow};
use std::io::Read;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        return Err(anyhow!("Usage: {} <json_file>", args[0]));
    }

    let content = read_file_content(&args[1])
        .with_context(|| format!("Failed to read file: {}", args[1]))?;

    let mut lexer = lexer::Lexer::new(content.chars());

    match lexer.parse() {
        Ok(_) => println!("The provided file is a valid JSON"),
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

fn read_file_content(file_path: &str) -> Result<String> {
    let mut content = String::new();
    std::fs::File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?
        .read_to_string(&mut content)
        .context("Failed to read file contents")?;

    Ok(content)
}
