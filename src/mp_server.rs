use crate::{Server, ServerResult};
use id3::Tag;
use mp_protocol::Response;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) trait MpServer {
    fn get_tags(&mut self, paths: &Path) -> ServerResult<Vec<Tag>>;
    fn add_files(&mut self, paths: &Vec<&Path>) -> ServerResult<Response>;
}

impl Server<'_> {
    pub(crate) fn add_files(&mut self, paths: &Vec<&Path>) -> ServerResult<Response> {
        let tags = self.get_all_tags(paths)?;
        self.db.insert_files(&tags)?;
        Ok(Response)
    }

    pub(crate) fn get_all_tags(
        &mut self,
        paths: &Vec<&Path>,
    ) -> ServerResult<Vec<(PathBuf, Tag, taglib::File)>> {
        let mut tags = vec![];
        for path in paths {
            let t = self.get_tags(path)?;
            tags.extend(t);
        }
        Ok(tags)
    }

    fn get_tags(
        &mut self,
        path: impl AsRef<Path>,
    ) -> ServerResult<Vec<(PathBuf, Tag, taglib::File)>> {
        let path = path.as_ref();
        // check if path exists
        fs::metadata(path)?;
        if path.is_dir() {
            let mut tags = vec![];
            for dir_entry in path.read_dir()? {
                let dir_entry = dir_entry?;
                let t = self.get_tags(dir_entry.path())?;
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
}

#[cfg(test)]
mod test {

    #[test]
    fn result_flat_map() {
        fn f(x: i32) -> Result<Vec<i32>, ()> {
            if x < 50 {
                Ok(vec![x - 1, x + 1])
            } else {
                Err(())
            }
        }

        let xs: Vec<_> = vec![10, 40, 70].into_iter().flat_map(f).collect();
        // flattening results just removes the failures
        assert_eq!(xs, vec![vec![9, 11], vec![39, 41]]);

        // flatmap with flatten works nice
        let xs: Vec<_> = vec![10, 40, 70].into_iter().flat_map(f).flatten().collect();
        assert_eq!(xs, vec![9, 11, 39, 41]);

        // collection fails the entire operation
        let xs = vec![10, 40, 70]
            .into_iter()
            .map(f)
            .collect::<Result<Vec<_>, _>>();
        assert_eq!(xs, Err(()));
    }
}
