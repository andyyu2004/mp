table! {
    album (album_id) {
        album_id -> Int4,
        artist_id -> Int4,
    }
}

table! {
    artist (artist_id) {
        artist_id -> Int4,
        artist_name -> Varchar,
    }
}

table! {
    track (track_id) {
        track_id -> Int4,
        title -> Varchar,
        album_id -> Int4,
    }
}

joinable!(album -> artist (artist_id));
joinable!(track -> album (album_id));

allow_tables_to_appear_in_same_query!(
    album,
    artist,
    track,
);
