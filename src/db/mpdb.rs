use super::*;
use crate::{Database, ServerResult};
use diesel::prelude::*;
use diesel::OptionalExtension;
use mp_protocol::JoinedTrack;
use std::path::PathBuf;

/// music player database
pub trait Mpdb {}

impl Database {
    pub fn get_track(&self, target_track_id: i32) -> ServerResult<JoinedTrack> {
        use super::schema::albums::columns::album_id;
        use super::schema::artists::columns::artist_id;
        use super::schema::{albums::dsl::*, artists::dsl::*, tracks::dsl::*};
        Ok(tracks
            .find(target_track_id)
            .inner_join(albums)
            .inner_join(
                artists.on(super::schema::artists::columns::artist_id
                    .eq(super::schema::albums::columns::artist_id)),
            )
            .select((
                track_id,
                title,
                lyrics,
                comments,
                genre,
                track_number,
                path,
                duration,
                bitrate,
                samplerate,
                channels,
                album_id,
                album_title,
                year,
                total_tracks,
                artist_id,
                artist_name,
            ))
            .get_result(&self.connection)?)
    }

    pub fn get_all(&self) -> ServerResult<Vec<JoinedTrack>> {
        use super::schema::albums::columns::album_id;
        use super::schema::artists::columns::artist_id;
        use super::schema::{albums::dsl::*, artists::dsl::*, tracks::dsl::*};

        let q = albums.inner_join(tracks).inner_join(artists).select((
            track_id,
            title,
            lyrics,
            comments,
            genre,
            track_number,
            path,
            duration,
            bitrate,
            samplerate,
            channels,
            album_id,
            album_title,
            year,
            total_tracks,
            artist_id,
            artist_name,
        ));

        let joined_tracks = q.load(&self.connection)?;
        Ok(joined_tracks)
    }

    pub fn insert_files(
        &mut self,
        data: &Vec<(PathBuf, id3::Tag, taglib::File)>,
    ) -> ServerResult<()> {
        for (path, tag, file) in data {
            let entry = InsertionEntry::from((path.as_path(), tag, file.audioproperties()?));
            let InsertionEntry {
                artist,
                album,
                track,
            } = entry;

            let artist = self.get_or_insert_artist(artist)?;
            let album = self.get_or_insert_album(album, artist.artist_id)?;
            let _track = self.get_or_insert_track(track, album.album_id)?;
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
