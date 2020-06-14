use crate::error::ServerResult;
use id3::Tag;
use std::{
    fs, path::{Path, PathBuf}
};

pub(crate) fn get_all_tags(paths: &Vec<&Path>) -> ServerResult<Vec<(PathBuf, Tag, taglib::File)>> {
    let mut tags = vec![];
    for path in paths {
        let t = get_tags(path)?;
        tags.extend(t);
    }
    Ok(tags)
}

fn get_tags(path: impl AsRef<Path>) -> ServerResult<Vec<(PathBuf, Tag, taglib::File)>> {
    let path = path.as_ref();
    // check if path exists
    fs::metadata(path)?;
    if path.is_dir() {
        let mut tags = vec![];
        for dir_entry in path.read_dir()? {
            let dir_entry = dir_entry?;
            let t = get_tags(dir_entry.path())?;
            tags.extend(t);
        }
        Ok(tags)
    } else if path.is_file() {
        let tag = Tag::read_from_path(path)?;
        let file = taglib::File::new(path)?;
        Ok(vec![(path.into(), tag, file)])
    } else {
        unreachable!()
    }
}
