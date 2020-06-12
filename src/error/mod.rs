use crate::util::join_display;
use std::fmt::{self, Display, Formatter};

pub type ProtocolResult<T> = Result<T, ProtocolError>;

#[macro_export]
macro_rules! impl_from(
    ($from:path, $for:ident, $variant:ident) => {
        impl From<$from> for $for {
            fn from(err: $from) -> Self {
                Self::$variant(err)
            }
        }
    }
);

#[derive(Debug)]
pub enum ParseError {
    InvalidOpcode(u8),
    InvalidEncoding(u8),
}

#[derive(Debug)]
pub struct DeserializationError;

impl_from!(DeserializationError, ProtocolError, DeserializationError);
impl_from!(std::io::Error, ProtocolError, IOError);
impl_from!(ParseError, ProtocolError, ParseError);
impl_from!(serde_json::error::Error, ProtocolError, JsonError);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidEncoding(encoding) => {
                write!(f, "Invalid encoding `{}` at index 0", encoding)
            }
            Self::InvalidOpcode(opcode) => write!(f, "Invalid opcode `{}` at index 1", opcode),
        }
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    ParseError(ParseError),
    Errors(Vec<ProtocolError>),
    DeserializationError(DeserializationError),
    IOError(std::io::Error),
    JsonError(serde_json::error::Error),
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errors(errors) => write!(f, "{}", join_display(errors, ",")),
            Self::ParseError(err) => write!(f, "{}", err),
            Self::JsonError(err) => write!(f, "{}", err),
            Self::DeserializationError(_) => write!(f, "Deserialization Error"),
            Self::IOError(err) => write!(f, "{}", err),
        }
    }
}
