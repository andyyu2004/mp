use super::schema::artists;

#[derive(Identifiable, Queryable, Debug, Deserialize)]
#[primary_key(artist_id)]
pub struct Artist {
    pub artist_id: i32,
    pub artist_name: String,
}

#[derive(Insertable)]
#[table_name = "artists"]
pub struct InsertableArtist {
    pub artist_name: String,
}
