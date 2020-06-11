use super::schema::albums;

#[derive(Identifiable, Deserialize, Debug, Queryable)]
#[primary_key(album_id)]
pub struct Album {
    pub album_id: i32,
    pub album_title: String,
    pub artist_id: i32,
    pub year: Option<i32>,
    pub total_tracks: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "albums"]
pub struct InsertableAlbum<'a> {
    pub album_title: &'a str,
    pub artist_id: i32,
    pub year: Option<i32>,
    pub total_tracks: Option<i32>,
}
