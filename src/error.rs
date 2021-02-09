use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, LstodoError>;

#[derive(Error, Debug)]
pub enum LstodoError {
    #[error("IO error {0}")]
    IO(#[from] io::Error),
    #[error("common HelixDB error")]
    Common,
    #[error("element not found")]
    NotFound,

    #[error("not a git repository")]
    NotRepo,
    #[error("git error {0}")]
    Git(#[from] git2::Error),
    #[error("walkdir failed {0}")]
    WalkDir(#[from] walkdir::Error),
}
