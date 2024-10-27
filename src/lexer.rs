
use crate::tokens::Tokens;
use crate::errors::Errors;
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: i16,
}

impl<'a> Lexer<'a>  {
    pub fn new(input: Chars<'a>) -> Self {
        Lexer {
            input: input.peekable(),
            line: 1,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Tokens>, Errors> {
        println!("{:?}",self.input);

        
        let mut tokens = Vec::<Tokens>::new();

        // File is empty.
        if self.peek().is_none() {
            Err(Errors::EmptyFile)?
        }

        while let Some(character) = self.read() {
            println!("tokens {:?}" , tokens);
            match character {
                '{' => tokens.push(Tokens::OpenCurlyBrace),
                '}' => {
                    if tokens[tokens.len()-1] == Tokens::Comma {
                        Err(
                            Errors::InvalidSyntax { 
                                expected_token: "x".to_string(), line: self.line 
                            })?
                    }
                    tokens.push(Tokens::ClosedCurlyBrace)
                },
                '"' => tokens.append(&mut self.parse_key_value()?),
                ',' => tokens.push(Tokens::Comma),
                c => Err(
                    Errors::UnexpectedToken { line: self.line, token: c }
                )?
            }
        }

        if tokens[0] != Tokens::OpenCurlyBrace {
            Err(Errors::InvalidSyntax { expected_token: "}".to_string(), line: self.line })?
        }
    
        if tokens[tokens.len() -1] == Tokens::Comma {
            Err(
                Errors::TrailingComma { line: self.line }
            )?
        }

        Ok(tokens)
    }

    fn read(&mut self) -> Option<char> {
        loop {
            if let Some(ch) = self.peek() {
                if Self::is_newline(ch) {
                    self.line += 1;
                    self.input.next();
                    continue;
                }
                if Self::is_whitespace(ch) {
                    self.input.next();
                    continue;
                }
                break;
            }
            break;
        }

        self.input.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    fn is_newline(char: char) -> bool {
        char == '\n' || char == '\r'
    }
    
    fn is_whitespace(char: char) -> bool {
        char == ' '
    }

    fn is_number(char: char) -> bool {
        ['0','1','2','3','4','5','6','7','8','9'].contains(&char)
    }

    fn parse_key_value(&mut self) -> Result<Vec<Tokens>, Errors> { 
        let mut tokens = Vec::<Tokens>::new();
        //Expecting "key"
        let key = self.parse_key()?;
        tokens.push(key.clone());

        //if object was nested, need to check if it was closed
        if key == Tokens::ClosedCurlyBrace {
            return Ok(tokens);
        }

        //expecting colon ":""
        if self.read().is_some_and(
            |character| character.to_string() == Tokens::Colon.literal()
        ) { 
            tokens.push(Tokens::Colon);
        } else {
            Err(Errors::InvalidSyntax { 
                expected_token: Tokens::Colon.literal(), 
                line: self.line 
            })?
        }

        if let Some(character) = self.read() {
            println!("charatcet: {:?}", character);
            println!("nested 118 nada");
            // if nested {
            //     println!("pasa a evaluar anidamiento");
            //     match character {
            //         '}' => {
            //             tokens.push(Tokens::ClosedCurlyBrace);
            //         },
            //         _ => ()
            //     }
            // }

            match character {
                //parse_key evaluates proper form of format "key"
                //which is the expected on keys and strings parts of a value.
                '"' => tokens.push(self.parse_key()?),
                't' => tokens.push(self.read_true()?),
                'f' => tokens.push(self.read_false()?),
                'n' => tokens.push(self.read_null()?),
                // '{' => {
                //     println!("matchea acá");
                //     tokens.push(Tokens::OpenCurlyBrace);
                //     println!("tokens {:?}" , tokens);
                //     tokens.append(&mut self.parse_key_value()?)
                // },
                '[' => tokens.push(self.read_array()?),
                c => {
                    if Self::is_number(c) {
                        tokens.push(self.read_number(c)?)
                    } else {
                        Err(Errors::InvalidSyntax {
                            line: self.line,
                            expected_token: String::from(c),
                        })?;
                    }
                },
            }
        }

        Ok(tokens)
    }

    fn read_array(&mut self) -> Result<Tokens, Errors> {
        let mut ended = true;

        for ch in self.input.by_ref() {
            match ch {
                ']' => {
                    ended = true;
                    break;
                }
                _ => todo!(),
            }
        }

        if !ended {
            Err(Errors::BadFormattedArray { line: self.line })?
        }

        Ok(Tokens::Array)
    }

    fn parse_key(&mut self) -> Result<Tokens, Errors> {
        let mut buffered_string = String::new();
        let mut key_ended = false;
        println!("parse key");


        for char in self.input.by_ref() {
            match char {
                '"' => {
                    key_ended = true;
                    break;
                },
                ':' => {
                    Err(Errors::MissingClosingQuote { line: self.line })?
                },
                ch => buffered_string.push(ch),
            }
        }

        if !key_ended {
            Err(Errors::UnterminatedString { line: self.line })?
        }
            println!("{:?}",buffered_string);
        Ok(Tokens::Literal(buffered_string))
    }

    fn read_number(&mut self, first_char: char) -> Result<Tokens, Errors> {
        let mut ended = false;
        let mut buffer = String::from(first_char);

        for number in self.input.by_ref() {
            if Self::is_number(number) {
                buffer.push(number);
            } else {
                ended = true;
                break;
            }
        }

        if !ended {
            Err(Errors::BadFormattedNumber { line: self.line })?
        }

        Ok(Tokens::Number(buffer.parse::<usize>().unwrap()))
    }
    
    fn read_specified_key(&mut self, mut buffer: String, stop_at: char, expected_token: Tokens) -> Result<Tokens, Errors> {
        let mut ended = false;

        for char in self.input.by_ref() {
            if char == stop_at {
                ended=true;
                break;
            } else {
                buffer.push(char)
            }
        }

        if !ended {
            Err(Errors::InvalidSyntax { expected_token: expected_token.literal(), line: self.line })?
        }

        if buffer != expected_token.literal() {
            Err(Errors::InvalidSyntax { expected_token: String::from("a crear error"), line: self.line })?
        }

        Ok(expected_token)
    }

    fn read_null(&mut self) -> Result<Tokens, Errors> {
        self.read_specified_key(String::from("n"), ',', Tokens::Null)
    }

    fn read_true(&mut self) -> Result<Tokens, Errors> {
        self.read_specified_key(String::from("t"), ',', Tokens::True)
    }

    fn read_false(&mut self) -> Result<Tokens, Errors> {
        self.read_specified_key(String::from("f"), ',', Tokens::False)
    }

}