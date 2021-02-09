use git2::Oid;
use std::cmp::Ordering;
use std::fs::Metadata;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Entry {
    pub path: String,
    pub line_number: u64,
    pub content: String,
    pub orig_commit: Option<Oid>,
    pub final_commit: Option<Oid>,
    pub modified: SystemTime,
}

impl Entry {
    pub fn new(
        path: String,
        line_number: u64,
        content: String,
        line_history: Option<(Oid, Oid)>,
        file_meta: Metadata,
    ) -> Self {
        // todo: rewrite this
        let (orig_commit, final_commit) = if let Some((orig_commit, final_commit)) = line_history {
            (Some(orig_commit), Some(final_commit))
        } else {
            (None, None)
        };

        let modified = file_meta.modified().unwrap();

        // strip leading whitespace
        let content = content.trim_start_matches(' ').to_string();

        Self {
            path,
            line_number,
            content,
            orig_commit,
            final_commit,
            modified,
        }
    }

    pub fn cmp_last_commit(lhs: &Self, rhs: &Self) -> Ordering {
        match (lhs.final_commit, rhs.final_commit) {
            (Some(lhs_oid), Some(rhs_oid)) => lhs_oid.cmp(&rhs_oid),
            _ => Ordering::Equal,
        }
    }

    pub fn cmp_first_commit(lhs: &Self, rhs: &Self) -> Ordering {
        match (lhs.orig_commit, rhs.orig_commit) {
            (Some(lhs_oid), Some(rhs_oid)) => lhs_oid.cmp(&rhs_oid),
            _ => Ordering::Equal,
        }
    }

    pub fn cmp_last_modified(lhs: &Self, rhs: &Self) -> Ordering {
        lhs.modified.cmp(&rhs.modified)
    }
}
