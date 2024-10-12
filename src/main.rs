use std::fs;

#[derive(PartialEq, Clone, Debug)]
enum Tokens {
    OpenCurlyBrace,
    ClosedCurlyBrace,
    Whitespace,
    Quote,
    Invalid,
}

#[derive(Debug)]
struct Lexer {
    raw_json: String,
    actual_position: u16,
}

impl Lexer {
    pub fn next_token(&mut self) -> Tokens {
        let mut next_token = Tokens::Invalid;

        while let Some(character) = self
            .raw_json
            .chars()
            .nth(usize::from(self.actual_position))
            .map(|c| (c))
        {
            match character {
                '{' => next_token = Tokens::OpenCurlyBrace,
                '}' => next_token = Tokens::ClosedCurlyBrace,
                ' ' => next_token = Tokens::Whitespace,
                '"' => next_token = Tokens::Quote,
                _ => break,
            }

            self.actual_position += 1;

            if next_token != Tokens::Invalid {
                break;
            }
        }

        next_token
    }
}

struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn parse_object(&mut self) -> bool {
        let mut inside_object = false;
        let mut open_key = false;
        let mut expect_semicolon = false;
        let mut expect_value = false;
    
    loop {
        let token = self.lexer.next_token();
        
        match token {
            Tokens::OpenCurlyBrace => {
                if inside_object {
                    return false;
                }
                inside_object = true;
            },
            Tokens::ClosedCurlyBrace => {
                if !inside_object {
                    return false;
                }
                return true;
            },
            Tokens::Quote => continue,
            Tokens::Whitespace => continue,
            Tokens::Invalid => return false,
        }
    }
    }
}

fn main() -> Result<(), ()> {
    let path = "/home/nlmansilla/dev/rust/json-parser/src/tests/step1/invalid.json".to_string();
    println!("Hello, world!");
    let mut _lexer = Lexer {
        raw_json: fs::read_to_string(path).unwrap(),
        actual_position: 0,
    };
    let mut parser = Parser { lexer: _lexer };

    if parser.parse_object() {
        Ok(())
    } else {
        Err(())
    }
}

#[test]
fn test_empty_file_should_fail() {}
