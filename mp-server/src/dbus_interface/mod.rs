//! dbus mpris interface implementation
use zbus::fdo;
use zbus::Connection;

#[derive(Default)]
struct MediaPlayer2Interface {}

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

#[derive(Default)]
struct MediaPlayer2PlayerInterface {}

#[zbus::dbus_interface(name = "org.mpris.MediaPlayer2.Player")]
impl MediaPlayer2PlayerInterface {
    fn next(&self) {
        println!("hello");
    }

    fn previous(&self) {
        println!("hello");
    }

    fn pause(&self) {
    }

    fn play_pause(&self) {
    }

    fn stop(&self) {
    }

    fn play(&self) {
    }

    fn seek(&self, offset: i64) {
    }

    fn set_position(&self, track_id: i32, pos: i32) {
    }

    fn open_uri(&self, uri: &str) {
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
        false
    }

    #[dbus_interface(property)]
    fn metadata(&self) -> f64 {
        todo!()
    }

    #[dbus_interface(property)]
    fn volume(&self) -> f64 {
        100.0
    }

    #[dbus_interface(property)]
    fn position(&self) -> f64 {
        0.0
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
pub async fn connect() -> Result<(), anyhow::Error> {
    let connection = Connection::new_session()?;
    fdo::DBusProxy::new(&connection)?
        .request_name("org.mpris.MediaPlayer2.mp", fdo::RequestNameFlags::ReplaceExisting.into())?;
    let mut object_server = zbus::ObjectServer::new(&connection);
    let interface = MediaPlayer2PlayerInterface::default();
    assert!(object_server.at("/org/mpris/MediaPlayer2", interface)?);
    loop {
        println!("handling");
        if let Err(err) = object_server.try_handle_next() {
            eprintln!("object_server failed to handle request: {}", err);
        }
    }
}
