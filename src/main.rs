use std::fs;

#[derive(PartialEq, Clone, Debug)]
enum Tokens {
    OpenCurlyBrace,
    ClosedCurlyBrace,
    Whitespace,
    Quote,
    Invalid,
    Colon,
    Comma,
    NewLine
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
            println!("{:?}", character);
            match character {
                '{' => next_token = Tokens::OpenCurlyBrace,
                '}' => next_token = Tokens::ClosedCurlyBrace,
                ' ' => next_token = Tokens::Whitespace,
                '"' => next_token = Tokens::Quote,
                ':' => next_token = Tokens::Colon,
                ',' => next_token = Tokens::Comma,
                '\n' => next_token = Tokens::NewLine,
                _ => next_token = Tokens::Whitespace,
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
        let mut expect_colon = false;
        let mut expect_opening_value = false;

        let mut expect_open_quote = false;
        let mut expect_close_quote = false;
        let mut expect_open_quote_value = false;
        let mut expect_close_quote_value = false;
        let mut expect_comma_or_closing_bracket = true;
    
    loop {
        let previous_position: usize = usize::from(self.lexer.actual_position);
        let token = self.lexer.next_token();
        let actual_position = usize::from(self.lexer.actual_position);
        println!("{:?}", token);
        //println!("{:?}", &self.lexer.raw_json[previous_position..actual_position]);

        match token {
            Tokens::OpenCurlyBrace => {
                if inside_object {
                    return false;
                }
                inside_object = true;
                expect_open_quote = true;
            },
            Tokens::ClosedCurlyBrace => {
                println!("{:?}", expect_opening_value);
                if !inside_object || expect_open_quote{
                    return false; // } sin corresponder
                }
                return true;
            },
            Tokens::Quote => {
                if !inside_object {
                    return false;
                }
                else if inside_object && expect_open_quote {
                    expect_open_quote = false;
                    expect_close_quote = true;
                }
                else if inside_object && expect_close_quote  {
                    expect_close_quote = false;
                    expect_colon = true;
                }
                else if inside_object && expect_close_quote  {
                    expect_close_quote = false;
                    expect_colon = true;
                }
                else if inside_object && expect_open_quote_value {
                    expect_open_quote_value = false;
                    expect_close_quote_value = true
                }
                else if inside_object && expect_close_quote_value {
                    expect_close_quote_value = false;
                    expect_comma_or_closing_bracket = true
                } else {
                    return false;
                }
            },
            Tokens::Colon => {
                //println!("{:?}", expect_colon);
                if !expect_colon {
                    return false;
                }
                expect_open_quote_value = true;
                expect_colon = false;
            },
            Tokens::Comma => {
                if expect_comma_or_closing_bracket {
                    expect_open_quote = true;
                    expect_colon = false;
                    expect_opening_value = false;
                    expect_comma_or_closing_bracket = false;
                } else {
                    return false;
                }
            }
            Tokens::Whitespace => continue,
            Tokens::NewLine => continue,
            Tokens::Invalid => return false,
        }
    }
    }
}

fn main() -> Result<(), ()> {
    let path = "/home/nlmansilla/dev/rust/json-parser/src/tests/step2/invalid2.json".to_string();
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
