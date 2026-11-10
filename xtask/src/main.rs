extern crate alloc;

mod generate;
mod task;

use std::env;
use std::io::{self, Write as _};
use std::process::ExitCode;

const USAGE: &str = "\
usage: cargo xtask <task>

tasks:
    ci               the generated drift check, fmt, clippy, tests, the fuzz smoke
                     run, the examples, and tigerstyle, in that order; CI runs this
                     and nothing else
    fuzz [name] [seed] [cases]
                     run one fuzzer, or \"smoke\" for a short run of every one
    generate         rewrite the generated modules from the pinned specifications
    specification    download Atlassian's current specifications into spec/
";

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).take(4).collect();

    assert!(arguments.len() <= 4, "a task takes at most three arguments after its name");

    let outcome = match arguments.first().map(String::as_str) {
        Some("ci") => task::checks_run(),
        Some("fuzz") => task::fuzz(arguments.get(1..).unwrap_or_default()),
        Some("generate") => task::generated(),
        Some("specification") => task::specification(),
        Some(_) | None => Err(USAGE.trim_end().to_owned()),
    };

    let Err(failure) = outcome else {
        return ExitCode::SUCCESS;
    };

    assert!(!failure.is_empty(), "a failed task says why");

    let written = writeln!(io::stderr().lock(), "xtask: {failure}");

    if written.is_err() {
        return ExitCode::from(2);
    }

    ExitCode::FAILURE
}
