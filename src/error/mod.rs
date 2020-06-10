use std::fmt::{self, Display, Formatter};
pub type ProtocolResult<T> = Result<T, ProtocolError>;

#[derive(Debug)]
pub enum ParseError {
    InvalidOpcode(u8),
    InvalidEncoding(u8),
}

#[derive(Debug)]
pub struct DeserializationError;

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

impl From<DeserializationError> for ProtocolError {
    fn from(err: DeserializationError) -> Self {
        Self::DeserializationError(err)
    }
}

impl From<std::io::Error> for ProtocolError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<ParseError> for ProtocolError {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err)
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    ParseError(ParseError),
    DeserializationError(DeserializationError),
    IOError(std::io::Error),
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "{}", err),
            Self::DeserializationError(_) => write!(f, "Deserialization Error"),
            Self::IOError(err) => write!(f, "{}", err),
        }
    }
}
