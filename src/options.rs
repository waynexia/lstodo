use clap::ArgMatches;
use std::path::Path;

/// Defines behavior options of this CLI.
#[derive(Debug)]
pub struct Options {
    pub root_dir: String,
    pub sort_by: SortByOption,
    pub reverse: bool,
}

impl Options {
    pub fn from_args(args: ArgMatches) -> Self {
        Self {
            root_dir: Self::parse_root_dir(&args),
            sort_by: Self::parse_sort_by(&args),
            reverse: Self::parse_reverse(&args),
        }
    }

    fn parse_root_dir(args: &ArgMatches) -> String {
        let root = args.value_of("dir").unwrap();
        let path = Path::new(root).canonicalize().unwrap();
        assert!(path.exists(), "{:?} do not exist.", path);
        path.to_str().unwrap().to_string()
    }

    fn parse_sort_by(args: &ArgMatches) -> SortByOption {
        SortByOption::from(args.value_of("sort"))
    }

    fn parse_reverse(args: &ArgMatches) -> bool {
        args.is_present("reverse")
    }
}

#[derive(Debug)]
pub enum SortByOption {
    /// no sort, default option
    None,
    FirstCommit,
    LastCommit,
    LastModified,
}

impl From<Option<&str>> for SortByOption {
    fn from(op: Option<&str>) -> Self {
        match op {
            None => Self::None,
            Some("fc") => Self::FirstCommit,
            Some("lc") => Self::LastCommit,
            Some("lm") => Self::LastModified,
            _ => unreachable!(),
        }
    }
}
