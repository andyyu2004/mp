use thiserror::Error;

pub type MediaResult<T> = Result<T, MediaError>;

#[derive(Debug, Error)]
pub enum MediaError {}
