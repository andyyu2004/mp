table! {
    albums (album_id) {
        album_id -> Integer,
        album_title -> Text,
        artist_id -> Integer,
        year -> Nullable<Integer>,
        total_tracks -> Nullable<Integer>,
    }
}

table! {
    artists (artist_id) {
        artist_id -> Integer,
        artist_name -> Text,
    }
}

table! {
    tracks (track_id) {
        track_id -> Integer,
        title -> Text,
        album_id -> Integer,
        lyrics -> Text,
        comments -> Text,
        genre -> Text,
        track_number -> Nullable<Integer>,
        path -> Text,
        duration -> Integer,
        bitrate -> Integer,
        samplerate -> Integer,
        channels -> Integer,
    }
}

joinable!(albums -> artists (artist_id));
joinable!(tracks -> albums (album_id));

allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    tracks,
);
