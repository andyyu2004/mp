use mp_protocol::JoinedTrack;

#[derive(Debug)]
pub(crate) struct MediaEvent {
    pub expected_response: MediaResponseKind,
    pub kind: MediaEventKind,
}

impl MediaEvent {
    pub fn new(expected_response: MediaResponseKind, kind: MediaEventKind) -> Self {
        Self {
            expected_response,
            kind,
        }
    }
}

#[derive(Debug)]
pub(crate) enum MediaResponseKind {
    /// no response expected
    None,
    Q,
    PlaybackState,
}

#[derive(Debug)]
pub(crate) enum MediaEventKind {
    /// sending just for a response
    None,
    Pause,
    Resume,
    TogglePlay,
    PlayPrev,
    PlayNext,
    ShuffleAll(Vec<JoinedTrack>),
    PlayTrack(JoinedTrack),
    QAppend(JoinedTrack),
    SetNextTrack(JoinedTrack),
    Seek(i64),
}
