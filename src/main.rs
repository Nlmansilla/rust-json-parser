mod errors;
mod lexer;
mod tokens;
mod tests;

use anyhow::{Context, Result};
use std::io::Read;

fn main() -> Result<()> {
    let path = std::env::args().nth(1).context("get file_path")?;
    let content = read_file_content(&path).expect("Error opening the file");

    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(_tokens) => {
            println!("The provided file is a valid JSON");
            Ok(())
        },
        Err(e) => {
            Err(e.into())
        }
    }

    
}

fn read_file_content(file_path: &str) -> Result<String> {
    let mut file = std::fs::File::open(file_path).context("opening json file")?;

    let mut content = String::new();

    file.read_to_string(&mut content)
        .context("read file contents")?;

    Ok(content)
}
