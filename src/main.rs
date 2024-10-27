mod errors;
mod lexer;
mod tokens;

use anyhow::{Context, Result};
use std::io::Read;

fn main() -> Result<()> {
    let path = std::env::args().nth(1).context("get file_path")?;
    let content = read_file_content(&path).expect("Error opening the file");

    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    dbg!(tokens);

    Ok(())
}

fn read_file_content(file_path: &str) -> Result<String> {
    let mut file = std::fs::File::open(file_path).context("opening json file")?;

    let mut content = String::new();

    file.read_to_string(&mut content)
        .context("read file contents")?;

    Ok(content)
}

#[test]
fn test_first_step_valid_file_should_work() {
    let content = "{}".to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(tokens) => {
            assert_eq!(1, 1);
            dbg!(tokens);
        }
        Err(_) => {
            panic!("Expected Ok(Vec<Tokens>), got an Error")
        }
    }
}

#[test]
fn test_first_step_invalid_file_should_work() {
    let content = "".to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(_tokens) => {
            panic!("Expected error, got OK")
        }
        Err(_) => {
            assert_eq!(1, 1);
        }
    }
}

#[test]
fn test_second_step_invalid_file_should_work() {
    let content = "{\"key\": \"value\",}".to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(_tokens) => {
            panic!("Expected error, got OK")
        }
        Err(_) => {
            assert_eq!(1, 1);
        }
    }
}

#[test]
fn test_second_step_invalid2_file_should_work() {
    let content = "{
        \"key\": \"value\",
        key2: \"value\"
      }"
    .to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(_tokens) => {
            panic!("Expected error, got OK")
        }
        Err(_) => {
            assert_eq!(1, 1);
        }
    }
}

#[test]
fn test_second_step_valid_file_should_work() {
    let content = "{\"key\": \"value\"}".to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(tokens) => {
            assert_eq!(1, 1);
            dbg!(tokens);
        }
        Err(_) => {
            panic!("Expected Ok(Vec<Tokens>), got an Error")
        }
    }
}

#[test]
fn test_second_step_valid2_file_should_work() {
    let content = "{
        \"key\": \"value\",
        \"key2\": \"value\"
      }"
    .to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(tokens) => {
            assert_eq!(1, 1);
            dbg!(tokens);
        }
        Err(_) => {
            panic!("Expected Ok(Vec<Tokens>), got an Error")
        }
    }
}

#[test]
fn test_third_step_valid_file_should_work() {
    let content = "{
        \"key1\": true,
        \"key2\": false,
        \"key3\": null,
        \"key4\": \"value\",
        \"key5\": 101
      }"
    .to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(tokens) => {
            assert_eq!(1, 1);
            dbg!(tokens);
        }
        Err(_) => {
            panic!("Expected Ok(Vec<Tokens>), got an Error")
        }
    }
}

#[test]
fn test_third_step_invalid_file_should_work() {
    let content = "{
        \"key1\": true,
        \"key2\": False,
        \"key3\": null,
        \"key4\": \"value\",
        \"key5\": 101
      }"
    .to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(_tokens) => {
            panic!("Expected error, got OK")
        }
        Err(_) => {
            assert_eq!(1, 1);
        }
    }
}

#[test]
fn test_fourth_step_invalid_file_should_work() {
    let content = "{
        \"key\": \"value\",
        \"key-n\": 101,
        \"key-o\": {
          \"inner key\": \"inner value\"
        },
        \"key-l\": ['list value']
      }"
    .to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(_tokens) => {
            panic!("Expected error, got OK")
        }
        Err(_) => {
            assert_eq!(1, 1);
        }
    }
}

#[test]
fn test_fourth_step_valid_file_should_work() {
    let content = "{
        \"key\": \"value\",
        \"key-n\": 101,
        \"key-o\": {},
        \"key-l\": []
      }"
    .to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(tokens) => {
            assert_eq!(1, 1);
            dbg!(tokens);
        }
        Err(_) => {
            panic!("Expected Ok(Vec<Tokens>), got an Error")
        }
    }
}

#[test]
fn test_fourth_step_valid2_file_should_work() {
    let content = "{
        \"key\": \"value\",
        \"key-n\": 101,
        \"key-o\": {
          \"inner key\": \"inner value\"
        },
        \"key-l\": [\"list value\"]
      }"
    .to_string();
    let mut lexer = lexer::Lexer::new(content.chars());

    let tokens = lexer.parse();

    match tokens {
        Ok(tokens) => {
            assert_eq!(1, 1);
            dbg!(tokens);
        }
        Err(_) => {
            panic!("Expected Ok(Vec<Tokens>), got an Error")
        }
    }
}
