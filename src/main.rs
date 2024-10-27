mod lexer;
mod tokens;
mod errors;

use std::io::Read;
use anyhow::{Context, Ok, Result};

fn main() -> Result<()> {
    let path = "/home/nlmansilla/dev/rust/json-parser-v2/src/tests/step4/valid.json".to_string();

    let mut content = read_file_content(&path).expect("todo mal");

    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    dbg!(tokens);

    Ok(())
}

fn read_file_content(file_path: &str) -> Result<String> {
   let mut file = std::fs::File::open(file_path).context("opening json file")?;

    let mut content = String::new();

    file.read_to_string(&mut content).context("read file contents")?;

    Ok(content)
}