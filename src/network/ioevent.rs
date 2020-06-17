pub(crate) enum IOEvent {
    // terminate connection to server
    Terminate,
    InitClient,
    FetchQ,
    UpdatePlaybackStatus,
    PlayPrev,
    PlayNext,
    TogglePlay,
    ShuffleAll,
    PlayTrack(i32),
    QueueAppend(i32),
    SetNextTrack(i32),
}
