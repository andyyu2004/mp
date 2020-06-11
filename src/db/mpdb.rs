use super::{Album, Artist, Entry, InsertableAlbum, InsertableArtist, InsertableTrack, Track};
use crate::{Database, ServerResult};
use diesel::prelude::*;
use diesel::OptionalExtension;
use std::path::PathBuf;

/// music player database
pub trait Mpdb {}

impl Database {
    pub fn insert_files(
        &mut self,
        data: &Vec<(PathBuf, id3::Tag, taglib::File)>,
    ) -> ServerResult<()> {
        for (path, tag, file) in data {
            let entry = Entry::from((path.as_path(), tag, file.audioproperties()?));
            let Entry {
                artist,
                album,
                track,
            } = entry;

            let artist = self.get_or_insert_artist(artist)?;
            dbg!(&artist);
            let album = self.get_or_insert_album(album, artist.artist_id)?;
            dbg!(&album);
            let track = self.get_or_insert_track(track, album.album_id)?;
            dbg!(&track);
        }

        Ok(())
    }

    // we can safely limit 1 due to the unique constraints
    pub(crate) fn get_or_insert_track(
        &mut self,
        mut insertable_track: InsertableTrack,
        track_album_id: i32,
    ) -> ServerResult<Track> {
        use super::schema::tracks::dsl::*;
        let predicate = title
            .eq(&insertable_track.title)
            .and(album_id.eq(track_album_id));
        let query = tracks.filter(predicate).limit(1);
        let track = query.get_result::<Track>(&self.connection).optional()?;

        Ok(match track {
            Some(track) => track,
            None => {
                insertable_track.album_id = track_album_id;
                diesel::insert_into(tracks)
                    .values(&insertable_track)
                    .execute(&self.connection)?;
                query.get_result::<Track>(&self.connection)?
            }
        })
    }

    pub(crate) fn get_or_insert_album(
        &mut self,
        mut insertable_album: InsertableAlbum,
        album_artist_id: i32,
    ) -> ServerResult<Album> {
        use super::schema::albums::dsl::*;
        let predicate = album_title
            .eq(&insertable_album.album_title)
            .and(artist_id.eq(album_artist_id));
        let query = albums.filter(predicate).limit(1);
        let album = query.get_result::<Album>(&self.connection).optional()?;

        Ok(match album {
            Some(album) => album,
            None => {
                // we must properly set the artist_id now that it is known
                insertable_album.artist_id = album_artist_id;
                diesel::insert_into(albums)
                    .values(&insertable_album)
                    .execute(&self.connection)?;
                query.get_result::<Album>(&self.connection)?
            }
        })
    }

    pub(crate) fn get_or_insert_artist(
        &mut self,
        insertable_artist: InsertableArtist,
    ) -> ServerResult<Artist> {
        use super::schema::artists::dsl::*;
        let query = artists
            .filter(artist_name.eq(&insertable_artist.artist_name))
            .limit(1);

        let artist = query.get_result::<Artist>(&self.connection).optional()?;
        dbg!(&artist);

        Ok(match artist {
            Some(artist) => artist,
            None => {
                diesel::insert_into(artists)
                    .values(&insertable_artist)
                    .execute(&self.connection)?;
                dbg!("succesfully inserted?");
                query.get_result::<Artist>(&self.connection)?
            }
        })
    }
}