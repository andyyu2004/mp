pub(crate) enum IOEvent {
    InitClient,
    UpdatePlaybackStatus,
    PlayTrack(i32),
}
