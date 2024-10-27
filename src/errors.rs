use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Errors {
    #[error("Empty JSON file is invalid.")]
    EmptyFile,

    #[error("[line {line:?}] Unexpected token {token:?}")]
    UnexpectedToken { line: u32, token: char },

    #[error("[line {line:?}] Missing closing quote.")]
    MissingClosingQuote { line: u32 },

    #[error("[line {line:?}] Missing closing quote.")]
    UnterminatedString { line: u32 },

    #[error("[line {line:?}] Bad syntax on number format.")]
    BadFormattedNumber { line: u32 },

    #[error("[line {line:?}] Bad syntaxm unexpected trailing comma.")]
    TrailingComma { line: u32 },

    #[error("[line {line:?}] Invalid sintax. Missing {expected_token:?} at line {line:?}.")]
    InvalidSyntax { expected_token: String, line: u32 },

    #[error("[line {line:?}] Bad formatted Array, Expected ].")]
    BadFormattedArray { line: u32 },
}
