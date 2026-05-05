use thiserror::Error;

#[derive(Debug, Error)]
pub enum AsciiError {
    #[error("serial read error: {0}")]
    ReadError(String),

    #[error("line has fewer than 16 fields: got {0}")]
    NotEnoughFields(usize),

    #[error("failed to parse float for field '{field}': '{value}'")]
    ParseFloat { field: &'static str, value: String },

    #[error("failed to parse u8 for field '{field}': '{value}'")]
    ParseU8 { field: &'static str, value: String },

    #[error("invalid unit for field '{field}': expected '{expected}', got '{actual}'")]
    InvalidUnit {
        field: &'static str,
        expected: &'static str,
        actual: String,
    },
}
