use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use mp_protocol::JoinedTrack;
use std::fmt::Debug;

pub(crate) trait Filter<'a> {
    type Item;
    /// return true if the item should be kept, false if it should be filtered out
    fn apply(&self, item: Self::Item) -> bool;
}

// empty regex "(?!)"
#[derive(Default)]
pub(crate) struct TrackFilter {
    matcher: SkimMatcherV2,
    track_filter: Option<String>,
    album_filter: Option<String>,
    artist_filter: Option<String>,
    genre_filter: Option<String>,
}

impl Debug for TrackFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "t: {:?}; album: {:?}; artist: {:?}; genre: {:?}",
            self.track_filter, self.album_filter, self.artist_filter, self.genre_filter
        )
    }
}

impl<T> From<T> for TrackFilter
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let mut track_filter = String::new();
        let mut album_filter = String::new();
        let mut artist_filter = String::new();
        let mut genre_filter = String::new();
        let s = s.as_ref();

        match &s[..2] {
            // search artist
            "/r" => artist_filter.push_str(&s[2..]),
            // search album
            "/l" => album_filter.push_str(&s[2..]),
            // search genre
            "/g" => genre_filter.push_str(&s[2..]),
            // search track
            "/t" => track_filter.push_str(&s[2..]),
            // search all
            _ => {
                track_filter.push_str(s);
                album_filter.push_str(s);
                genre_filter.push_str(s);
                artist_filter.push_str(s);
            }
        };

        Self {
            matcher: SkimMatcherV2::default(),
            track_filter: Some(track_filter),
            album_filter: Some(album_filter),
            artist_filter: Some(artist_filter),
            genre_filter: Some(genre_filter),
        }
    }
}

impl<'a> Filter<'a> for TrackFilter {
    type Item = &'a JoinedTrack;

    fn apply(&self, track: Self::Item) -> bool {
        // fuzzy_returns 0 on when the pattern is empty and we don't want to consider that a match
        // so empty string matches nothing, None matches anything
        let t = self
            .track_filter
            .as_ref()
            .map(|t| self.matcher.fuzzy_match(&track.title, t) > Some(0))
            .unwrap_or(true);
        let l = self
            .album_filter
            .as_ref()
            .map(|l| self.matcher.fuzzy_match(&track.album_title, l) > Some(0))
            .unwrap_or(true);
        let r = self
            .artist_filter
            .as_ref()
            .map(|r| self.matcher.fuzzy_match(&track.artist_name, r) > Some(0))
            .unwrap_or(true);
        let g = self
            .genre_filter
            .as_ref()
            .map(|g| self.matcher.fuzzy_match(&track.genre, g) > Some(0))
            .unwrap_or(true);

        t || l || r || g
    }
}
