use git2::{BlameOptions, Oid, Repository};
use std::path::Path;

use crate::error::{LstodoError, Result};

pub struct GitContext {
    repo: Repository,
}

impl GitContext {
    pub fn try_new<P: AsRef<Path>>(root: P) -> Result<Self> {
        let repo = Repository::discover(root).map_err(|_| LstodoError::NotRepo)?;

        Ok(GitContext { repo })
    }

    pub fn is_ignored(&self, path: &Path) -> Result<bool> {
        Ok(self.repo.status_file(path)?.is_ignored())
    }

    #[inline]
    pub fn work_dir(&self) -> &Path {
        self.repo.workdir().unwrap()
    }

    /// path should be a file. otherwise will panic.
    pub fn get_line_oid(&self, path: &Path, line_number: usize) -> Oid {
        let relative_path = path.strip_prefix(self.work_dir()).unwrap();
        println!("path: {:?}, relative path: {:?}", path, relative_path);
        self.repo
            .blame_file(relative_path, None)
            .unwrap()
            .get_line(line_number)
            .unwrap()
            .orig_commit_id()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ignore_entry() {
        let ctx = GitContext::try_new("/home/wayne/repo/lstodo/src").unwrap();
        // let path = Path::new("target/CACHEDIR.TAG");
        let path = Path::new(".git");

        assert_eq!(true, ctx.is_ignored(path).unwrap());
    }

    #[test]
    fn blame_file() {
        let ctx = GitContext::try_new("/home/wayne/repo/lstodo").unwrap();
        let path = Path::new("/home/wayne/repo/lstodo/src/regex.rs");

        println!("oid: {:?}", ctx.get_line_oid(path, 1));
    }
}
