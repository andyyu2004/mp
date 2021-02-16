use crate::db::InsertionEntry;
use crate::error::ServerResult;
use lazy_static::lazy_static;
use maplit::hashset;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

lazy_static! {
    pub(crate) static ref ALLOWED_FILE_TYPES: std::collections::HashSet<&'static str> =
        hashset! { "mp3" };
}

/// iterate all top level paths and call get_tags each and collect the results
pub(crate) fn get_all_tags(paths: &Vec<&Path>) -> ServerResult<Vec<InsertionEntry>> {
    let mut tags = vec![];
    for path in paths {
        let ts = get_tags(path)?;
        tags.extend(ts);
    }
    Ok(tags)
}

fn get_tags(path: impl AsRef<Path>) -> ServerResult<Vec<InsertionEntry>> {
    let path = path.as_ref();
    // check if path exists
    let meta = fs::metadata(path)?;
    if meta.is_dir() {
        let mut tags = vec![];
        for dir_entry in path.read_dir()? {
            tags.extend(get_tags(dir_entry?.path())?);
        }
        Ok(tags)
    } else {
        // if the file does not a have an appropriate extension then ignore it
        let ext = path.extension().and_then(OsStr::to_str);
        if ext.is_none() || !ALLOWED_FILE_TYPES.contains(ext.unwrap()) {
            return Ok(vec![]);
        }
        let tag = id3::Tag::read_from_path(path)?;
        let file = taglib::File::new(path)?;
        let entry = InsertionEntry::from((path, &tag, file.audioproperties()?));
        Ok(vec![entry])
    }
}
