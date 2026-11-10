use std::env;
use std::io::{self, Write as _};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use confluence::fuzz;

const CASE_COUNT_DEFAULT: u32 = 100_000;

const USAGE: &str = "\
usage: cargo run --release --bin fuzz -- <fuzzer> [seed] [cases]

fuzzers:
    adf         flatten randomly generated Atlassian Document Format trees
    client      answer requests with random failures and check the retry policy
    page        parse randomly generated, hostile pages
    text        cut randomly generated strings at a byte budget
    url         fill a request with hostile parameter values
    smoke       run every fuzzer briefly, at the seed and case count the tests use

The seed defaults to the clock and is always reported, so a failure can be
replayed by passing the seed back.
";

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).take(4).collect();

    assert!(arguments.len() <= 4, "one argument past the three a fuzzer takes is collected");

    if arguments.len() > 3 {
        return stderr_write("a fuzzer takes a name, a seed, and a case count at most\n");
    }

    let Some(name) = arguments.first() else {
        return stderr_write(USAGE);
    };

    if name == "smoke" {
        return smoke_run(&arguments);
    }

    let seed = match arguments.get(1).map(|text| (text, text.parse())) {
        None => seed_from_clock(),
        Some((_text, Ok(seed))) => seed,
        Some((text, Err(cause))) => {
            return stderr_write(&format!("the seed \"{text}\" is not a whole number: {cause}\n"));
        }
    };

    let case_count = match arguments.get(2).map(|text| (text, text.parse())) {
        None => CASE_COUNT_DEFAULT,
        Some((_text, Ok(0))) => return stderr_write("a fuzz run checks at least one case\n"),
        Some((_text, Ok(case_count))) => case_count,
        Some((text, Err(cause))) => {
            return stderr_write(&format!(
                "the case count \"{text}\" is not a whole number: {cause}\n",
            ));
        }
    };

    let Some(case_count_run) = fuzz::run(name, seed, case_count) else {
        return stderr_write(&format!(
            "unknown fuzzer {name}: expected one of {}, or smoke\n",
            fuzz::NAMES.join(", "),
        ));
    };

    assert_eq!(case_count_run, case_count, "every case asked for ran");

    stdout_write(&format!("{name}: {case_count_run} cases at seed {seed}\n"))
}

fn seed_from_clock() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(1, |since| u64::from(since.subsec_nanos()) ^ since.as_secs())
}

fn smoke_run(arguments: &[String]) -> ExitCode {
    assert_eq!(
        arguments.first().map(String::as_str),
        Some("smoke"),
        "a smoke run is asked by name",
    );

    if arguments.len() > 1 {
        return stderr_write(
            "smoke runs at the seed and case count the tests use, and takes neither\n",
        );
    }

    let mut report = String::new();

    for name in fuzz::NAMES {
        let Some(case_count) = fuzz::run(name, fuzz::SEED_SMOKE, fuzz::CASE_COUNT_SMOKE) else {
            return stderr_write(&format!("the registry names {name}, which does not run\n"));
        };

        report.push_str(name);
        report.push_str(": ");
        report.push_str(&case_count.to_string());
        report.push_str(" cases at seed ");
        report.push_str(&fuzz::SEED_SMOKE.to_string());
        report.push('\n');
    }

    assert_eq!(report.lines().count(), fuzz::NAMES.len(), "each fuzzer reports one line");
    assert!(report.ends_with('\n'), "the report ends its last line");

    stdout_write(&report)
}

fn stderr_write(text: &str) -> ExitCode {
    assert!(!text.is_empty(), "a report says something");
    assert!(text.ends_with('\n'), "a report ends its last line");

    let written = io::stderr().lock().write_all(text.as_bytes());

    if written.is_err() {
        return ExitCode::from(2);
    }

    ExitCode::FAILURE
}

fn stdout_write(text: &str) -> ExitCode {
    assert!(!text.is_empty(), "a report says something");
    assert!(text.ends_with('\n'), "a report ends its last line");

    let written = io::stdout().lock().write_all(text.as_bytes());

    if written.is_err() {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
