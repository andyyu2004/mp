use mp_protocol::JoinedTrack;

#[derive(Debug)]
pub(crate) struct MediaEvent {
    pub expected_response: MediaResponseKind,
    pub kind: MediaEventKind,
}

impl MediaEvent {
    pub fn new(expected_response: MediaResponseKind, kind: MediaEventKind) -> Self {
        Self { expected_response, kind }
    }
}

#[derive(Debug)]
pub(crate) enum MediaResponseKind {
    /// no response expected
    None,
    Queue,
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
    QueueAppend(JoinedTrack),
    SetNextTrack(JoinedTrack),
    ChangeVolume(i32),
    Seek(i64),
}
