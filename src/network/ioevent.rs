pub(crate) enum IOEvent {
    InitClient,
    FetchQ,
    UpdatePlaybackStatus,
    PlayTrack(i32),
    TogglePlay,
    QueueAppend(i32),
}
