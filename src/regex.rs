use regex::RegexSet;
use std::{borrow::Borrow, convert::TryInto, io::BufReader};
use std::{fs::File, io::BufRead};
use walkdir::{DirEntry, WalkDir};

use crate::entry::Entry;
use crate::error::Result;
use crate::git::GitContext;

pub const DEFAULT_REGEXS: &[&str] = &["(?i)//\\s*todo"];

pub struct RegexSearcherBuilder {
    root: String,
    rules: Vec<String>,
    git_ctx: Option<GitContext>,
}

impl RegexSearcherBuilder {
    pub fn new(root: String) -> Self {
        Self {
            root,
            rules: vec![],
            git_ctx: None,
        }
    }

    pub fn add_rules(mut self, rules: &[&str]) -> Self {
        for rule in rules {
            self.rules.push(rule.to_string());
        }
        self
    }

    pub fn git_ctx(mut self, git_ctx: GitContext) -> Self {
        self.git_ctx = Some(git_ctx);
        self
    }

    pub fn build(self) -> RegexSearcher {
        let root = WalkDir::new(self.root);
        let regex = RegexSet::new(self.rules).unwrap();

        RegexSearcher {
            regex,
            root,
            git_ctx: self.git_ctx,
        }
    }
}

pub struct RegexSearcher {
    regex: RegexSet,
    root: WalkDir,
    git_ctx: Option<GitContext>,
}

impl RegexSearcher {
    pub fn search(self) -> Result<()> {
        let mut results = vec![];

        let RegexSearcher {
            regex,
            root,
            git_ctx,
        } = self;

        for entry in root
            .into_iter()
            .filter_entry(|e| Self::file_filter(e, &git_ctx).unwrap())
        {
            let entry = entry?;
            if entry.metadata()?.is_dir() {
                continue;
            }

            let file = File::open(entry.path()).unwrap();
            let reader = BufReader::new(file);
            let mut line_number = 1;
            for line in reader.lines() {
                if let Ok(line) = line {
                    if regex.matches(&line).matched_any() {
                        results.push(Entry::new(
                            entry.path().to_string_lossy().into(),
                            line_number,
                            line,
                            git_ctx
                                .borrow()
                                .as_ref()
                                .map(|ctx| ctx.get_line_oid(entry.path(), line_number as usize))
                                .flatten(),
                        ));
                    }
                } else {
                    break;
                }
                line_number += 1;
            }
        }

        println!("results:\n{:?}", results);

        Ok(())
    }

    fn file_filter(entry: &DirEntry, git_ctx: &Option<GitContext>) -> Result<bool> {
        // ignore hidden file
        if entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
        {
            return Ok(false);
        }

        // ignore files covered in .gitignore
        if let Some(git_ctx) = git_ctx {
            if !entry.metadata()?.is_dir() && git_ctx.is_ignored(entry.path())? {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_regexs() {
        let cases = vec![
            ("//todo", true),
            ("// todo", true),
            ("// TODO-123", true),
            ("//   TodO", true),
            ("not_a_todo", false),
        ];

        let regex = RegexSet::new(DEFAULT_REGEXS).unwrap();

        for (input, expected) in cases {
            assert_eq!(regex.matches(input).matched_any(), expected)
        }
    }

    #[test]
    fn temp_main() {
        let searcher = RegexSearcherBuilder::new("/home/wayne/repo/lstodo".to_owned())
            .add_rules(DEFAULT_REGEXS)
            .git_ctx(GitContext::with_dir("/home/wayne/repo/lstodo/src"))
            .build();

        searcher.search().unwrap();
    }
}
