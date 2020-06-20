use crate::decoding;
use crate::{Decode, Decoder, Encode, Encoder, Encoding, Opcode, ProtocolError, ProtocolResult};
use std::convert::TryFrom;
use std::path::Path;

/// request is an enumeration of all requests that a client can send to the server
#[derive(Debug, PartialEq)]
pub enum Request<'r> {
    AddFile(Vec<&'r Path>),
    PlayTrack(i32),
    QAppend(i32),
    SetNextTrack(i32),
    Seek(i64),
    FetchTracks,
    FetchQ,
    FetchPlaybackState,
    ResumePlayback,
    PausePlayback,
    TogglePlay,
    PlayPrev,
    PlayNext,
    ShuffleAll,
}

/// implement decoding of a request from bytes of any encoding (encoding is encoded in the first byte of the buffer)
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
            Self::AddFile(paths) => encoder.encode_add_file(paths),
            Self::PlayTrack(track_id) => encoder.encode_f_track(Opcode::PlayTrk, *track_id),
            Self::QAppend(track_id) => encoder.encode_f_track(Opcode::QAppend, *track_id),
            Self::SetNextTrack(track_id) => encoder.encode_f_track(Opcode::SetNxtTrk, *track_id),
            Self::Seek(t) => encoder.encode_seek(*t),
            Self::FetchTracks => encoder.encode_opcode(Opcode::FetchTrk),
            Self::FetchPlaybackState => encoder.encode_opcode(Opcode::FetchPlaybackState),
            Self::ResumePlayback => encoder.encode_opcode(Opcode::ResumePlayback),
            Self::PausePlayback => encoder.encode_opcode(Opcode::PausePlayback),
            Self::TogglePlay => encoder.encode_opcode(Opcode::TogglePlay),
            Self::FetchQ => encoder.encode_opcode(Opcode::QFetch),
            Self::PlayPrev => encoder.encode_opcode(Opcode::PlayPrv),
            Self::PlayNext => encoder.encode_opcode(Opcode::PlayNxt),
            Self::ShuffleAll => encoder.encode_opcode(Opcode::ShuffleAll),
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
            Opcode::AddFile => Self::AddFile(decoder.decode_add_file(&buf)?),
            Opcode::PlayTrk => Self::PlayTrack(decoder.decode_i32(&buf)?),
            Opcode::QAppend => Self::QAppend(decoder.decode_i32(&buf)?),
            Opcode::SetNxtTrk => Self::SetNextTrack(decoder.decode_i32(&buf)?),
            Opcode::Seek => Self::Seek(decoder.decode_i64(&buf)?),
            Opcode::FetchTrk => Self::FetchTracks,
            Opcode::FetchPlaybackState => Self::FetchPlaybackState,
            Opcode::TogglePlay => Self::TogglePlay,
            Opcode::ResumePlayback => Self::ResumePlayback,
            Opcode::PausePlayback => Self::PausePlayback,
            Opcode::QFetch => Self::FetchQ,
            Opcode::PlayPrv => Self::PlayPrev,
            Opcode::PlayNxt => Self::PlayNext,
            Opcode::ShuffleAll => Self::ShuffleAll,
        })
    }
}
