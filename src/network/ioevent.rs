pub(crate) enum IOEvent {
    // terminate connection to server
    Terminate,
    InitClient,
    FetchQ,
    UpdatePlaybackStatus,
    PlayPrev,
    PlayNext,
    PlayTrack(i32),
    TogglePlay,
    QueueAppend(i32),
    SetNextTrack(i32),
}
