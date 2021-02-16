use crate::error::ServerResult;
use ignore::{DirEntry, Walk};
use std::path::Path;

/// creates a copy of the `src` directory into `dest` directory where
/// the folder structure has been "canonicalized"
/// i.e. each track now has the following path:
/// <dest>/<artist>/<album>/<track-number> - <title>.<ext>
pub fn canonicalize(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> ServerResult<()> {
    let src = src.as_ref().canonicalize()?;
    let dest = dest.as_ref().canonicalize()?;
    for entry in Walk::new(src) {
        match entry {
            Ok(entry) => canonicalize_file(entry, &dest)?,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };
    }

    Ok(())
}

pub fn canonicalize_file(entry: DirEntry, dest: impl AsRef<Path>) -> ServerResult<()> {
    let tag = id3::Tag::read_from_path(entry.path())?;
    let artist = tag.artist().unwrap_or("unknown artist");
    let album = tag.album().unwrap_or("unknown album");
    let title = tag.title().unwrap_or("unknown track");
    let track = tag.track().unwrap_or(0);
    let ext = entry.path().extension().map_or("", |os| os.to_str().unwrap());
    let new_path =
        format!("{}/{}/{}/{} - {}.{}", dest.as_ref().display(), artist, album, track, title, ext);
    dbg!(new_path);
    Ok(())
}
