use id3::{Tag, Version};
use mp_protocol::ProtocolResult;
use std::fs;
use std::path::Path;

pub(crate) fn add_files(paths: &Vec<&Path>) -> ProtocolResult<()> {
    for &path in paths {
        add_file(path)?;
    }
    Ok(())
}

fn add_file(path: &Path) -> ProtocolResult<()> {
    println!("reading file: `{:?}`", path);
    // check if path exists
    fs::metadata(path)?;
    if path.is_dir() {
        for dir_entry in path.read_dir()? {
            let dir_entry = dir_entry?;
            add_file(dir_entry.path().as_path())?;
        }
    } else if path.is_file() {
        let tag = Tag::read_from_path(path);
        println!("{:?}", tag);
    }
    Ok(())
}
