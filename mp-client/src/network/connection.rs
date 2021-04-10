use super::IOEvent;
use crate::Client;
use mp_protocol::ProtocolResult;
use std::io;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

pub(crate) struct Connection {
    socket: UnixStream,
    pub(crate) tx: Sender<IOEvent>,
    rx: Receiver<IOEvent>,
    pub(crate) client: Arc<Mutex<Client>>,
}

impl Connection {
    pub async fn new(
        path: impl AsRef<Path>,
        client: Arc<Mutex<Client>>,
        tx: Sender<IOEvent>,
        rx: Receiver<IOEvent>,
    ) -> io::Result<Self> {
        let socket = UnixStream::connect(path).await?;
        Ok(Self { socket, client, tx, rx })
    }

    pub async fn listen(&mut self) -> ProtocolResult<()> {
        while let Ok(event) = self.rx.recv() {
            match event {
                IOEvent::UpdatePlaybackStatus => self.dispatch_fetch_playback_state().await,
                IOEvent::InitClient => self.init_client().await,
                IOEvent::TogglePlay => Ok(self.dispatch_toggle_play().await?),
                IOEvent::FetchQ => Ok(self.dispatch_fetch_q().await?),
                IOEvent::PlayPrev => Ok(self.dispatch_play_prev().await?),
                IOEvent::PlayNext => Ok(self.dispatch_play_next().await?),
                IOEvent::ShuffleAll => Ok(self.dispatch_shuffle_all().await?),
                IOEvent::ChangeVolume(delta) => Ok(self.dispatch_change_volume(delta).await?),
                IOEvent::PlayTrack(track_id) => Ok(self.dispatch_play_track(track_id).await?),
                IOEvent::QueueAppend(track_id) => Ok(self.dispatch_queue_append(track_id).await?),
                IOEvent::Seek(t) => Ok(self.dispatch_seek(t).await?),
                IOEvent::SetNextTrack(track_id) =>
                    Ok(self.dispatch_set_next_track(track_id).await?),
                IOEvent::Terminate => {
                    self.close().await?;
                    break;
                }
            }?;
        }
        Ok(())
    }

    pub async fn init_client(&mut self) -> ProtocolResult<()> {
        trace!("init_client");
        self.dispatch_fetch_tracks().await
    }

    pub async fn send(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.socket.write_u32(bytes.len() as u32).await?;
        self.socket.write_all(bytes).await?;
        Ok(())
    }

    pub async fn recv(&mut self) -> io::Result<Vec<u8>> {
        let msg_len = self.socket.read_u32().await? as usize;
        let mut buf = vec![0u8; msg_len];
        self.socket.read_exact(&mut buf).await?;
        Ok(buf)
    }
}
