use mp_protocol::{impl_from, ProtocolError};

pub(crate) type ClientResult<T> = Result<T, ClientError>;

impl_from!(std::io::Error, ClientError, IO);
impl_from!(ProtocolError, ClientError, Protocol);
impl_from!(std::sync::mpsc::RecvError, ClientError, MpscRecv);

#[derive(Debug)]
pub enum ClientError {
    IO(std::io::Error),
    Protocol(ProtocolError),
    MpscRecv(std::sync::mpsc::RecvError),
}
