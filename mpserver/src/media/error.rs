use mp_protocol::impl_from;

pub type MediaResult<T> = Result<T, MediaError>;

#[derive(Debug)]
pub enum MediaError {}
