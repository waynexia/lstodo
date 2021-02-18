use git2::{Commit, Oid, Repository};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::Path;

use crate::error::Result;

pub struct GitContext<'commit> {
    /// Option for no git repository.
    /// In this case all operation will preform empty action. Like return `None` for getter.
    repo: Option<Repository>,
    commits: HashMap<Oid, Commit<'commit>>,
}

impl<'commit> GitContext<'commit> {
    /// Create a empty (no git repository specified) [GitContext].
    pub fn new() -> Self {
        Self {
            repo: None,
            commits: HashMap::new(),
        }
    }

    /// Initialize [GitContext] with given directory.
    /// If this directory is not a git repository, a empty context will be returned.
    pub fn with_dir<P: AsRef<Path>>(&mut self, root: P) {
        if let Ok(repo) = Repository::discover(root) {
            self.repo = Some(repo);
        } else {
            // todo: print warning
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

            repo.blame_file(path, None).ok().map(|blame| {
                let hunk = blame.get_line(line_number).unwrap();
                (hunk.orig_commit_id(), hunk.final_commit_id())
            })
        } else {
            None
        }
    }

    /// # Panic
    /// If given Oid doesn't exist.
    pub fn find_commit<'a>(&'a mut self, oid: Oid) -> Option<&Commit<'commit>>
    where
        'commit: 'a,
    {
        let lifetime_commit = PhantomData::<&'commit usize>::default();
        if let Some(repo) = &self.repo {
            let commit = self.commits.entry(oid).or_insert_with(|| {
                let commit = repo.find_commit(oid).unwrap();
                extend_commit_lifetime(commit, lifetime_commit)
            });
            Some(commit)
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
    fn convert_path<'b>(&self, path: &'b Path) -> &'b Path {
        if path.is_absolute() {
            path.strip_prefix(self.work_dir().unwrap()).unwrap()
        } else {
            path
        }
    }
}

/// Extend the lifetime binds with a `Commit`.
///
/// # Background
/// `Commit` will holds a lifetime used to free underlying resource. This
/// lifetime parameter is set implicit when create a `Commit` object. For
/// example in [GitContext::find_commit], we call `find_commit()` of
/// `Repository` to get a `Commit` with a reference of `Repository`. And
/// the returning `Commit`'s lifetime is equal to `Repository`'s reference
/// (with rust's lifetime elision rule).
///
/// However, at least in our use case, `Commit`'s lifetime needn't to equal
/// to one reference instance of a `Repository` object. It can be at least as
/// long as the lifetime of `Repository` object's, not just one reference.
///
/// The concrete example (requirement) from this project is, we have a list
/// of `Oid` to find their corresponding `Commit`. This list and a `Repository`
/// object is bound into one struct (with lifetime `'a`). `Commit`s will be cached
/// into a container (with lifetime `'commit`) for performance. Then we iterating
/// on this `Oid` list's entries and use `Repository` to query `Commit`. And in
/// each loop of this iteration, we use a `Oid` entry to query a `Repository`'s
/// reference. Both entry and reference have lifetime `'entry`. Since we may change
/// cache's content, the repository's reference should be mutable. And we cannot
/// keep two mutable reference at the same time. So in each loop we actually
/// re-borrow that `Repository`. Hence we have *`'a` : (many different, non-overlapping) `'entry`*.
/// In the other hand, `Commit`s in the cache should have the same lifetime `'commit`. This
/// is also the lifetime that we will get from the cache.
///
/// But what we really get from `find_commit()` is `'entry`. This is why we need
/// this function to extend `'entry` to `'commit`.
///
/// # Safety
/// Let's continue with above three lifetimes. From the definition of [Printer] (the
/// struct that bounds `Repository`(`GitContext`) and `Oid` list described above). We
/// have: *`'a` = `'commit'`*. So *`'commit` : (many) `'entry`*.
///
/// And it is safe to extend `Commit`'s lifetime from `'entry` to `'commit`.
fn extend_commit_lifetime<'a, 'b, 'commit>(
    commit: Commit<'a>,
    _lifetime_commit: PhantomData<&'commit usize>,
) -> Commit<'b>
where
    'b: 'a,
    'commit: 'a,
{
    unsafe { std::mem::transmute::<Commit<'a>, Commit<'b>>(commit) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ignore_entry() {
        let mut ctx = GitContext::new();
        ctx.with_dir("/home/wayne/repo/lstodo/src");
        let path = Path::new(".git");

        assert_eq!(true, ctx.is_ignored(path).unwrap());
    }

    #[test]
    fn blame_file() {
        let mut ctx = GitContext::new();
        ctx.with_dir("/home/wayne/repo/lstodo/src");
        let path = Path::new("/home/wayne/repo/lstodo/src/regex.rs");

        println!("oid: {:?}", ctx.get_line_history(path, 1));
    }
}
