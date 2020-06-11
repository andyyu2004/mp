use mp_protocol::{util, ProtocolError};
use std::fmt::{self, Display, Formatter};

pub(crate) type ServerResult<T> = Result<T, ServerError>;

impl From<taglib::FileError> for ServerError {
    fn from(err: taglib::FileError) -> Self {
        Self::FileError(err)
    }
}
impl From<id3::Error> for ServerError {
    fn from(err: id3::Error) -> Self {
        Self::TagError(err)
    }
}

impl From<ProtocolError> for ServerError {
    fn from(err: ProtocolError) -> Self {
        Self::ProtocolError(err)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<diesel::result::Error> for ServerError {
    fn from(err: diesel::result::Error) -> Self {
        Self::DbError(err)
    }
}

#[derive(Debug)]
pub enum ServerError {
    ProtocolError(ProtocolError),
    Errors(Vec<ServerError>),
    TagError(id3::Error),
    FileError(taglib::FileError),
    IOError(std::io::Error),
    DbError(diesel::result::Error),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Errors(errors) => write!(f, "{}", util::join_display(errors, ",")),
            Self::TagError(err) => write!(f, "{}", err),
            Self::ProtocolError(err) => write!(f, "{}", err),
            Self::FileError(err) => write!(f, "{:?}", err),
            Self::IOError(err) => write!(f, "{}", err),
            Self::DbError(err) => write!(f, "{}", err),
        }
    }
}
