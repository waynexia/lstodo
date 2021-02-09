use regex::RegexSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use walkdir::{DirEntry, WalkDir};

use crate::entry::Entry;
use crate::error::Result;
use crate::git::GitContext;
use crate::options::Options;

pub const DEFAULT_REGEXS: &[&str] = &["(?i)//\\s*todo"];

pub struct RegexSearcherBuilder {
    root: String,
    rules: Vec<String>,
    git_ctx: GitContext,
}

impl RegexSearcherBuilder {
    pub fn with_options(options: &Options) -> Self {
        let git_ctx = GitContext::with_dir(options.root_dir.clone());
        Self {
            root: options.root_dir.clone(),
            rules: vec![],
            git_ctx,
        }
    }

    pub fn add_rules(mut self, rules: &[&str]) -> Self {
        for rule in rules {
            self.rules.push(rule.to_string());
        }
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
    git_ctx: GitContext,
}

impl RegexSearcher {
    pub fn search(self) -> Result<Vec<Entry>> {
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
                            git_ctx.get_line_history(entry.path(), line_number as usize),
                            entry.metadata()?,
                        ));
                    }
                } else {
                    break;
                }
                line_number += 1;
            }
        }

        Ok(results)
    }

    fn file_filter(entry: &DirEntry, git_ctx: &GitContext) -> Result<bool> {
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
        if !entry.metadata()?.is_dir() && git_ctx.is_ignored(entry.path())? {
            return Ok(false);
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
}
