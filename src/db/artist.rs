use super::schema::artists;

#[derive(Queryable, Debug, Deserialize)]
pub struct Artist {
    pub artist_id: i32,
    pub artist_name: String,
}

#[derive(Insertable)]
#[table_name = "artists"]
pub struct InsertableArtist<'a> {
    pub artist_name: &'a str,
}
