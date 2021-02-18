use termion::{color, style};

use crate::entry::Entry;
use crate::git::GitContext;
use crate::options::{Options, SortByOption};

const SHORT_HASH_LENGTH: usize = 7;
const HALF_TAB: &str = "  ";
const TAB: &str = "    ";

/// Responsible for all post-processing like sort, filter, format and output.
pub struct Printer<'a> {
    entries: Vec<Entry>,
    git_ctx: &'a mut GitContext<'a>,
    options: Options,
}

impl<'a> Printer<'a> {
    pub fn new(entries: Vec<Entry>, git_ctx: &'a GitContext<'a>, options: Options) -> Self {
        // todo: receive `&'a mut GitContext<'a>` directly.
        #[allow(clippy::cast_ref_to_mut)]
        let git_ctx: &'a mut GitContext<'a> =
            unsafe { &mut *(git_ctx as *const _ as *mut GitContext<'a>) };

        Self {
            entries,
            git_ctx,
            options,
        }
    }

    pub fn prepare(mut self) -> Self {
        self.sort();

        self
    }

    pub fn print(mut self) {
        let entries = std::mem::take(&mut self.entries);
        for entry in entries {
            println!(
                "{}{}* ->{} {}:{}",
                color::Fg(color::Blue),
                style::Bold,
                style::Reset,
                entry.path,
                entry.line_number
            );

            self.print_commit(&entry);

            println!("\n{}{}{}\n", HALF_TAB, TAB, entry.content)
        }
    }
}

impl<'a> Printer<'a> {
    fn sort(&mut self) {
        let options = &self.options;
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

    fn print_commit<'entry>(&'entry mut self, entry: &'entry Entry)
    where
        'a: 'entry,
    {
        if let Some(oid) = entry.orig_commit {
            if let Some(commit) = self.git_ctx.find_commit(oid) {
                unsafe {
                    println!(
                        "{}{}since{} {}{}{}: {}{}",
                        HALF_TAB,
                        style::Bold,
                        style::Reset,
                        color::Fg(color::Yellow),
                        oid.to_string().get_unchecked(0..SHORT_HASH_LENGTH),
                        style::Reset,
                        commit.summary().unwrap(),
                        style::Reset
                    );
                }
            }
        }
    }
}
