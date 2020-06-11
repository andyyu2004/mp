use super::{InsertableAlbum, InsertableArtist, InsertableTrack};
use std::path::Path;

const UNKNOWN_TITLE: &str = "unknown title";
const UNKNOWN_ALBUM: &str = "unknown album";
const UNKNOWN_ARTIST: &str = "unknown artist";
const UNKNOWN_GENRE: &str = "unknown genre";

/// wrapper for artist, album, track
pub(crate) struct InsertionEntry<'a> {
    pub track: InsertableTrack<'a>,
    pub album: InsertableAlbum<'a>,
    pub artist: InsertableArtist<'a>,
}

impl<'a> From<(&'a Path, &'a id3::Tag, taglib::AudioProperties<'a>)> for InsertionEntry<'a> {
    fn from(
        (path, tag, properties): (&'a Path, &'a id3::Tag, taglib::AudioProperties<'a>),
    ) -> Self {
        let title = tag.title().unwrap_or(UNKNOWN_TITLE);
        let album_title = tag.album().unwrap_or(UNKNOWN_ALBUM);
        let artist_name = tag.artist().unwrap_or(UNKNOWN_ARTIST);
        let genre = tag.genre().unwrap_or(UNKNOWN_GENRE);
        let lyrics = tag
            .lyrics()
            .next()
            .map(|lyrics| lyrics.description.as_str())
            .unwrap_or("");
        let comments: Vec<&str> = tag.comments().map(|c| c.description.as_str()).collect();
        let comments = comments.join(";");
        let track_number = tag.track().map(|i| i as i32);
        let year = tag.year().map(|i| i as i32);
        let total_tracks = tag.total_tracks().map(|i| i as i32);
        let _pictures: Vec<_> = tag.pictures().collect();

        let artist = InsertableArtist { artist_name };

        let duration = properties.length() as i32;
        let bitrate = properties.bitrate() as i32;
        let samplerate = properties.samplerate() as i32;
        let channels = properties.channels() as i32;
        let path = path.to_str().unwrap();

        // make sure to set the artist_id properly later
        let album = InsertableAlbum {
            album_title,
            artist_id: -1,
            year,
            total_tracks,
        };

        let track = InsertableTrack {
            title,
            album_id: -1,
            lyrics,
            genre,
            comments,
            track_number,
            path,
            duration,
            bitrate,
            samplerate,
            channels,
            //pictures,
        };

        Self {
            artist,
            album,
            track,
        }
    }
}
