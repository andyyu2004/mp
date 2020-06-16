pub(crate) enum IOEvent {
    // terminate connection to server
    Terminate,
    InitClient,
    FetchQ,
    UpdatePlaybackStatus,
    PlayTrack(i32),
    TogglePlay,
    QueueAppend(i32),
}
