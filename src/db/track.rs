use super::schema::tracks;
use diesel::{Insertable, Queryable};

#[derive(Identifiable, Debug, Queryable, Deserialize)]
#[primary_key(track_id)]
pub struct Track {
    pub track_id: i32,
    pub title: String,
    pub album_id: i32,
    pub lyrics: String,
    pub comments: String,
    pub genre: String,
    pub track_number: Option<i32>,
    //pictures: Vec<&'a id3::frame::Picture>,
    pub path: String,
    pub duration: i32,
    pub bitrate: i32,
    pub samplerate: i32,
    pub channels: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "tracks"]
pub struct InsertableTrack {
    pub title: String,
    pub album_id: i32,
    pub lyrics: String,
    pub comments: String,
    pub genre: String,
    pub track_number: Option<i32>,
    //pictures: Vec<&'a id3::frame::Picture>,
    pub path: String,
    pub duration: i32,
    pub bitrate: i32,
    pub samplerate: i32,
    pub channels: i32,
}
