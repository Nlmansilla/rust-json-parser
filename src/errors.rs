use thiserror::Error;

#[derive(Error, Debug)]
pub (crate) enum Errors {
    #[error("Empty JSON file is invalid.")]
    EmptyFile,

    #[error("[line {line:?}] Unexpected token {token:?}")]
    UnexpectedToken { line: i16, token: char },

    #[error("[line {line:?}] Missing closing quote.")]
    MissingClosingQuote { line: i16 },

    #[error("[line {line:?}] Missing closing quote.")]
    UnterminatedString { line: i16 },

    #[error("[line {line:?}] Bad syntax on number format.")]
    BadFormattedNumber { line: i16 },

    #[error("[line {line:?}] Bad syntaxm unexpected trailing comma.")]
    TrailingComma { line: i16 },

    #[error("[line {line:?}] Invalid sintax. Missing {expected_token:?} at line {line:?}.")]
    InvalidSyntax {expected_token: String, line: i16},

    #[error("[line {line:?}] Bad formatted Array, Expected ].")]
    BadFormattedArray { line: i16 },
}