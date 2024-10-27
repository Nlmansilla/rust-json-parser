#[derive(PartialEq, Clone, Debug)]
pub enum Tokens {
    OpenCurlyBrace,
    ClosedCurlyBrace,
    Literal(String),
    Number(usize),
    Array,
    Colon,
    Comma,
    Null,
    True,
    False,
}

impl Tokens {
    pub(crate) fn literal(&self) -> String {
        match self {
            Tokens::OpenCurlyBrace => String::from("{"),
            Tokens::ClosedCurlyBrace => String::from("}"),
            Tokens::Literal(s) => s.clone(),
            Tokens::Number(n) => n.to_string(),
            Tokens::Colon => String::from(":"),
            Tokens::Comma => String::from(","),
            Tokens::Null => String::from("null"),
            Tokens::Array => String::from("[]"),
            Tokens::True => String::from("true"),
            Tokens::False => String::from("false"),
        }
    }
}
