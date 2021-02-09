use clap::{App, Arg};

mod entry;
mod error;
mod git;
mod options;
mod printer;
mod regex;

use crate::options::Options;
use crate::printer::Printer;
use crate::regex::{RegexSearcherBuilder, DEFAULT_REGEXS};

fn main() {
    let matches = App::new("")
        .arg(
            // unimplemented
            Arg::with_name("output")
                .short("c")
                .long("output")
                .help("set output file"),
        )
        .arg(
            Arg::with_name("sort")
                .short("s")
                .long("sort")
                .takes_value(true)
                .possible_values(&["fc", "lc", "lm"])
                .hide_possible_values(true)
                .help(
                    "sort by [ fc | lc | lm ]. Default in descending order.\n\
                    The options are \"first commit\", \"last commit\" and \"last modified\".",
                ),
        )
        .arg(
            Arg::with_name("reverse")
                .long("reverse")
                .help("reverse the output list"),
        )
        .arg(
            // unimplemented
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("display verbose message"),
        )
        .arg(
            // unimplemented
            Arg::with_name("oneline")
                .long("oneline")
                .help("display in one line"),
        )
        .arg(
            // unimplemented
            Arg::with_name("since commit")
                .long("since")
                .help("ignore commits earlier than given"),
        )
        .arg(
            // unimplemented
            Arg::with_name("before commit")
                .long("before")
                .help("ignore commits later than given"),
        )
        .arg(
            Arg::with_name("dir")
                .help("the base dir of scanning.")
                .default_value("./"),
        )
        .get_matches();

    let options = Options::from_args(matches);
    let results = RegexSearcherBuilder::with_options(&options)
        .add_rules(DEFAULT_REGEXS)
        .build()
        .search()
        .unwrap();

    Printer::new()
        .entries(results)
        .prepare(&options)
        .print(&options);
    // println!("results: {:#?}", results);
}
