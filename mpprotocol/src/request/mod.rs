use crate::decoding;
use crate::{Decode, Decoder, Encode, Encoder, Encoding, Opcode, ProtocolError, ProtocolResult};
use std::convert::TryFrom;
use std::path::Path;

/// request is an enumeration of all requests that a client can send to the server
#[derive(Debug, PartialEq)]
pub enum Request<'r> {
    AddFile(Vec<&'r Path>),
    PlayTrack(i32),
    QueueAppend(i32),
    SetNextTrack(i32),
    Canonicalize(&'r Path, &'r Path),
    ChangeVolume(i32),
    Seek(i64),
    FetchTracks,
    FetchQueue,
    FetchPlaybackState,
    ResumePlayback,
    PausePlayback,
    TogglePlay,
    PlayPrev,
    PlayNext,
    ShuffleAll,
}

/// implement decoding of a request from bytes of any encoding
/// (encoding is encoded in the first byte of the buffer)
impl<'r> TryFrom<&'r [u8]> for Request<'r> {
    type Error = ProtocolError;

    fn try_from(buf: &'r [u8]) -> ProtocolResult<Self> {
        let encoding = Encoding::from_u8(buf[0])?;
        let mut decoder = decoding::get_decoder(encoding);
        Request::decode(&buf[1..], decoder.as_mut())
    }
}

impl<'r> Encode for Request<'r> {
    fn encode<E>(&self, mut encoder: E) -> Result<E::Ok, E::Error>
    where
        E: Encoder,
    {
        match self {
            Request::AddFile(paths) => encoder.encode_op_files(Opcode::AddFile, paths),
            Request::PlayTrack(track_id) => encoder.encode_op_i32(Opcode::PlayTrack, *track_id),
            Request::QueueAppend(track_id) => encoder.encode_op_i32(Opcode::QueueAppend, *track_id),
            Request::ChangeVolume(delta) => encoder.encode_op_i32(Opcode::ChangeVolume, *delta),
            Request::SetNextTrack(track_id) =>
                encoder.encode_op_i32(Opcode::SetNextTrack, *track_id),
            Request::Seek(t) => encoder.encode_op_i64(Opcode::Seek, *t),
            Request::FetchTracks => encoder.encode_opcode(Opcode::FetchTrack),
            Request::FetchPlaybackState => encoder.encode_opcode(Opcode::FetchPlaybackState),
            Request::ResumePlayback => encoder.encode_opcode(Opcode::ResumePlayback),
            Request::PausePlayback => encoder.encode_opcode(Opcode::PausePlayback),
            Request::TogglePlay => encoder.encode_opcode(Opcode::TogglePlay),
            Request::FetchQueue => encoder.encode_opcode(Opcode::FetchQueue),
            Request::PlayPrev => encoder.encode_opcode(Opcode::PlayPrev),
            Request::PlayNext => encoder.encode_opcode(Opcode::PlayNext),
            Request::ShuffleAll => encoder.encode_opcode(Opcode::ShuffleAll),
            Request::Canonicalize(src, dst) => {
                encoder.encode_path(src)?;
                encoder.encode_path(dst)
            }
        }
    }
}

impl<'r> Decode<'r> for Request<'r> {
    fn decode<D>(buf: &'r [u8], mut decoder: D) -> Result<Self, D::Error>
    where
        D: Decoder,
    {
        let opcode = decoder.decode_opcode(buf[0])?;
        let buf = &buf[1..];
        Ok(match opcode {
            Opcode::AddFile => Self::AddFile(decoder.decode_paths(&buf)?),
            Opcode::PlayTrack => Self::PlayTrack(decoder.decode_i32(&buf)?),
            Opcode::QueueAppend => Self::QueueAppend(decoder.decode_i32(&buf)?),
            Opcode::SetNextTrack => Self::SetNextTrack(decoder.decode_i32(&buf)?),
            Opcode::Seek => Self::Seek(decoder.decode_i64(&buf)?),
            Opcode::ChangeVolume => Self::ChangeVolume(decoder.decode_i32(&buf)?),
            Opcode::FetchTrack => Self::FetchTracks,
            Opcode::FetchPlaybackState => Self::FetchPlaybackState,
            Opcode::TogglePlay => Self::TogglePlay,
            Opcode::ResumePlayback => Self::ResumePlayback,
            Opcode::PausePlayback => Self::PausePlayback,
            Opcode::FetchQueue => Self::FetchQueue,
            Opcode::PlayPrev => Self::PlayPrev,
            Opcode::PlayNext => Self::PlayNext,
            Opcode::ShuffleAll => Self::ShuffleAll,
        })
    }
}
