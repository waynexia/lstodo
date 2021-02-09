use termion::{color, style};

use crate::entry::Entry;
use crate::options::{Options, SortByOption};

/// Responsible for all post-processing like sort, filter, format and output.
pub struct Printer {
    entries: Vec<Entry>,
}

impl Printer {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn entries(mut self, entries: Vec<Entry>) -> Self {
        self.entries = entries;

        self
    }

    pub fn prepare(mut self, options: &Options) -> Self {
        self.sort(options);

        self
    }

    pub fn print(self, _options: &Options) {
        for entry in self.entries {
            println!(
                "{}{}* ->{} {}:{}",
                color::Fg(color::Blue),
                style::Bold,
                style::Reset,
                entry.path,
                entry.line_number
            );

            if let Some(oid) = entry.orig_commit {
                println!(
                    "  {}since{}: {}{}{}",
                    style::Bold,
                    style::Reset,
                    color::Fg(color::Yellow),
                    oid,
                    style::Reset
                );
            }

            println!("\n  {}\n", entry.content)
        }
    }

    fn sort(&mut self, options: &Options) {
        match options.sort_by {
            SortByOption::None => {}
            SortByOption::FirstCommit => self.entries.sort_by(Entry::cmp_first_commit),
            SortByOption::LastCommit => self.entries.sort_by(Entry::cmp_last_commit),
            SortByOption::LastModified => self.entries.sort_by(Entry::cmp_last_modified),
        };

        if options.reverse {
            self.entries.reverse();
        }
    }
}
