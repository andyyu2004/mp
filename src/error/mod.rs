use mp_protocol::{impl_from, util, ProtocolError};
use std::fmt::{self, Display, Formatter};

pub(crate) type ServerResult<T> = Result<T, ServerError>;

impl_from!(taglib::FileError, ServerError, FileError);
impl_from!(id3::Error, ServerError, TagError);
impl_from!(ProtocolError, ServerError, ProtocolError);
impl_from!(std::io::Error, ServerError, IOError);
impl_from!(diesel::result::Error, ServerError, DbError);
impl_from!(Vec<ServerError>, ServerError, Errors);

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
