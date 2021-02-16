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
    Seek(i64),
    PlayTrack(i32),
    QueueAppend(i32),
    SetNextTrack(i32),
    ChangeVolume(i32),
}
