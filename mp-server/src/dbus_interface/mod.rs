//! dbus mpris interface implementation
use crate::error::ServerResult;
use crate::server::Server;
use futures::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use zbus::fdo;
use zbus::Connection;

struct MediaPlayer2Interface {
    server: Arc<Mutex<Server>>,
}

impl MediaPlayer2Interface {
    pub fn new(server: Arc<Mutex<Server>>) -> Self {
        Self { server }
    }
}

#[zbus::dbus_interface(name = "org.mpris.MediaPlayer2")]
impl MediaPlayer2Interface {
    fn raise(&self) {
    }

    fn quit(&self) {
    }

    fn can_quit(&self) -> bool {
        false
    }

    fn can_raise(&self) -> bool {
        false
    }

    fn has_track_list(&self) -> bool {
        false
    }

    fn identity(&self) -> &str {
        "mp"
    }

    fn supported_uri_schemes(&self) -> &[&str] {
        &[]
    }

    fn supported_mime_types(&self) -> &[&str] {
        &[]
    }
}

struct MediaPlayer2PlayerInterface {
    server: Arc<Mutex<Server>>,
}

impl MediaPlayer2PlayerInterface {
    pub fn new(server: Arc<Mutex<Server>>) -> Self {
        Self { server }
    }
}

impl MediaPlayer2PlayerInterface {
    fn with_server<F, R>(&self, f: impl FnOnce(&mut Server) -> F)
    where
        F: Future<Output = ServerResult<R>>,
    {
        let mut server = futures::executor::block_on(self.server.lock());
        futures::executor::block_on(f(&mut server)).unwrap();
    }
}

macro_rules! block {
    ($self:ident,$method:ident) => {{
        let mut server = futures::executor::block_on($self.server.lock());
        futures::executor::block_on(server.$method()).unwrap();
    }};
}

#[zbus::dbus_interface(name = "org.mpris.MediaPlayer2.Player")]
impl MediaPlayer2PlayerInterface {
    fn next(&self) {
        block!(self, handle_play_next)
    }

    fn previous(&self) {
        block!(self, handle_play_prev)
    }

    fn pause(&self) {
        block!(self, handle_pause_playback)
    }

    fn play_pause(&self) {
        block!(self, handle_toggle_play)
    }

    fn stop(&self) {
        block!(self, handle_pause_playback)
    }

    fn play(&self) {
        block!(self, handle_resume_playback)
    }

    fn seek(&self, offset: i64) {
        todo!()
    }

    fn set_position(&self, track_id: i32, pos: i32) {
        todo!()
    }

    fn open_uri(&self, uri: &str) {
        todo!()
    }

    #[dbus_interface(signal)]
    fn seeked(&self, pos: i32) -> Result<(), zbus::Error>;

    #[dbus_interface(property)]
    fn playback_status(&self) -> &str {
        "todo"
    }

    #[dbus_interface(property)]
    fn loop_status(&self) -> &str {
        "single"
    }

    #[dbus_interface(property)]
    fn rate(&self) -> f64 {
        1.0
    }

    #[dbus_interface(property)]
    fn shuffle(&self) -> bool {
        true
    }

    // TODO not really the right type, and obvious data is bs
    #[dbus_interface(property)]
    fn metadata(&self) -> Vec<(&str, &str)> {
        vec![("mpris:trackid", "heerwer")]
    }

    #[dbus_interface(property)]
    fn volume(&self) -> f64 {
        100.0
    }

    #[dbus_interface(property)]
    fn position(&self) -> i64 {
        0
    }

    #[dbus_interface(property)]
    fn minimum_rate(&self) -> f64 {
        1.0
    }

    #[dbus_interface(property)]
    fn maximum_rate(&self) -> f64 {
        1.0
    }

    #[dbus_interface(property)]
    fn can_go_next(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_go_previous(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_play(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_pause(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_seek(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_control(&self) -> bool {
        false
    }
}

#[tokio::main]
pub(crate) async fn connect(server: Arc<Mutex<Server>>) -> Result<(), anyhow::Error> {
    let connection = Connection::new_session()?;
    fdo::DBusProxy::new(&connection)?
        .request_name("org.mpris.MediaPlayer2.mp", fdo::RequestNameFlags::ReplaceExisting.into())?;
    let mut object_server = zbus::ObjectServer::new(&connection);
    assert!(
        object_server
            .at("/org/mpris/MediaPlayer2", MediaPlayer2Interface::new(Arc::clone(&server)))?
    );
    assert!(object_server.at("/org/mpris/MediaPlayer2", MediaPlayer2PlayerInterface::new(server))?);
    loop {
        if let Err(err) = object_server.try_handle_next() {
            eprintln!("object_server failed to handle request: {}", err);
        }
    }
}
