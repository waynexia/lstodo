mod entry;
mod error;
mod git;
mod regex;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("lstodo")
        .arg(
            Arg::with_name("output")
                .short("c")
                .long("output")
                .help("set output file"),
        )
        .arg(
            Arg::with_name("sort")
                .short("s")
                .long("sort")
                .help("sort by"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("display verbose message"),
        )
        .arg(
            Arg::with_name("oneline")
                .long("oneline")
                .help("display in one line"),
        )
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("dir")
                .help("working dir"),
        );
}
