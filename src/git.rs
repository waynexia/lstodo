use git2::{Oid, Repository};
use std::path::Path;

use crate::error::Result;

pub struct GitContext {
    /// Option for no git repository.
    /// In this case all operation will preform empty action. Like return `None` for getter.
    repo: Option<Repository>,
}

impl GitContext {
    /// Create a empty (no git repository specified) [GitContext].
    ///
    /// This can be used as `Default::default()` for [GitContext].
    #[allow(unused)]
    pub fn empty() -> Self {
        Self { repo: None }
    }

    /// Initialize [GitContext] with given directory.
    /// If this directory is not a git repository, a empty context will be returned.
    pub fn with_dir<P: AsRef<Path>>(root: P) -> Self {
        if let Ok(repo) = Repository::discover(root) {
            GitContext { repo: Some(repo) }
        } else {
            // todo: print warning
            GitContext { repo: None }
        }
    }

    /// Check whether given path is covered by `.gitignore` file.
    pub fn is_ignored(&self, path: &Path) -> Result<bool> {
        if let Some(repo) = &self.repo {
            let path = self.convert_path(path);
            if path.as_os_str().is_empty() {
                Ok(false)
            } else {
                Ok(repo.status_file(path)?.is_ignored())
            }
        } else {
            Ok(false)
        }
    }

    /// Get the `OID` of one line. If given file is not contained in the
    /// repository yet (i.e. a uncommitted  new file), `Oid::zero()` will
    /// be returned.
    ///
    /// # Panic
    /// - If `path` is not a file.
    /// - If the line number exceeding the real number.
    pub fn get_line_history(&self, path: &Path, line_number: usize) -> Option<(Oid, Oid)> {
        if let Some(repo) = &self.repo {
            let path = self.convert_path(path);

            // if let Ok(blame) = repo.blame_file(path, None) {
            //     Some(blame.get_line(line_number).unwrap().orig_commit_id())
            // } else {
            //     Some(Oid::zero())
            // }

            repo.blame_file(path, None).ok().map(|blame| {
                let hunk = blame.get_line(line_number).unwrap();
                (hunk.orig_commit_id(), hunk.final_commit_id())
            })
        } else {
            None
        }
    }

    /// Get the `.git`'s root dir
    ///
    /// Empty context will return `None` instead.
    #[inline]
    fn work_dir(&self) -> Option<&Path> {
        self.repo.as_ref().map(|repo| repo.workdir().unwrap())
    }

    /// Since `git2` only works with relative path, we need to check and do
    /// conversation if necessary.
    ///
    /// # Panic
    /// If `self` is empty.
    fn convert_path<'a>(&self, path: &'a Path) -> &'a Path {
        if path.is_absolute() {
            path.strip_prefix(self.work_dir().unwrap()).unwrap()
        } else {
            path
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ignore_entry() {
        let ctx = GitContext::with_dir("/home/wayne/repo/lstodo/src");
        let path = Path::new(".git");

        assert_eq!(true, ctx.is_ignored(path).unwrap());
    }

    #[test]
    fn blame_file() {
        let ctx = GitContext::with_dir("/home/wayne/repo/lstodo");
        let path = Path::new("/home/wayne/repo/lstodo/src/regex.rs");

        println!("oid: {:?}", ctx.get_line_history(path, 1));
    }
}
