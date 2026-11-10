use std::fs;
use std::path::{Path, PathBuf};

const DIRECTORY_COUNT_MAX: u32 = 64;
const ENTRY_POINTS: [&str; 1] = ["lib.rs"];
const FILE_BYTES_MAX: u64 = 1024 * 1024;
const FILE_COUNT_MAX: u32 = 128;
const FUNCTION_LINE_COUNT_MAX: u64 = 70;
const LIST_INDENT: u64 = 4;
const MACRO_OPENERS: [&str; 5] = ["!(", "!([", "!({", "![", "!{"];
const MATCH_COUNT_MAX: u32 = 4_096;

const OWNERSHIP: [(&str, &[&str]); 3] = [
    ("SystemTime::now", &["client.rs"]),
    ("thread::sleep", &["agent.rs"]),
    ("ureq", &["agent.rs", "wire.rs"]),
];

const ROOTS: [&str; 4] = ["examples", "src", "tests", "xtask"];
const SIGNATURE_LINE_COUNT_MAX: u32 = 12;

#[derive(Clone, Copy, Debug)]
struct Block {
    end: u64,
    indent: u64,
    start: u64,
}

#[derive(Clone, Copy, Debug)]
struct Neighbours {
    above: Span,
    below: Span,
}

#[derive(Clone, Copy, Debug)]
struct Signature {
    indent: u64,
    start: u64,
}

#[derive(Clone, Copy, Debug)]
struct Span {
    first: u64,
    last: u64,
}

#[derive(Debug)]
struct Source {
    name: String,
    path: PathBuf,
    text: String,
}

fn blank_failures(source: &Source, lines: &[&str]) -> Vec<String> {
    let inside = strings_inside(lines);
    let mut failures: Vec<String> = Vec::new();

    for (index, line) in (0_u64..).zip(lines) {
        let is_blank = line.trim().is_empty() && !inside.contains(&index);

        if !is_blank {
            continue;
        }

        let previous = line_at(lines, index.checked_sub(1));
        let following = line_at(lines, Some(index.strict_add(1)));
        let place = format!("{}:{}", source.path.display(), index.strict_add(1));

        if code_of(previous).trim_end().ends_with('{') {
            failures.push(format!("{place} opens a block with a blank line"));
        }

        if following.trim_start().starts_with('}') {
            failures.push(format!("{place} closes a block with a blank line"));
        }

        if previous.trim().is_empty() {
            failures.push(format!("{place} is a second blank line in a row"));
        }
    }

    assert!(failures.len() <= lines.len().strict_mul(3), "each line fails at most three ways");
    assert!(inside.len() <= lines.len(), "at most every line sits inside a string");

    failures
}

fn blocks_find(lines: &[&str], inside: &[u64]) -> Vec<Block> {
    let mut found =
        vec![Block { end: u64::try_from(lines.len()).expect("a small file"), indent: 0, start: 0 }];

    for (index, line) in (0_u64..).zip(lines) {
        let is_opener = code_of(line).trim_end().ends_with('{') && !inside.contains(&index);

        if !is_opener {
            continue;
        }

        let indent = indent_of(line);
        let skipped = lines.iter().skip(index.strict_add(1).try_into().expect("a small offset"));

        for (offset, candidate) in (index.strict_add(1)..).zip(skipped) {
            let is_close =
                indent_of(candidate) == indent && candidate.trim_start().starts_with('}');

            if is_close {
                found.push(Block {
                    end: offset,
                    indent: indent.strict_add(4),
                    start: index.strict_add(1),
                });

                break;
            }
        }
    }

    assert!(!found.is_empty(), "every file is at least its own block");
    assert!(found.iter().all(|block| block.start <= block.end), "a block opens before it closes");

    found
}

fn body_is_present(lines: &[&str], start: u64) -> bool {
    assert!(!lines.is_empty(), "a signature is read from a file with lines");

    assert!(
        start < u64::try_from(lines.len()).expect("a small file"),
        "a signature is in the file",
    );

    let skipped = lines.iter().skip(start.try_into().expect("a small offset"));

    for line in skipped.take(SIGNATURE_LINE_COUNT_MAX.try_into().expect("a small window")) {
        let trimmed = line.trim_end();

        if trimmed.ends_with('{') {
            return true;
        }

        if trimmed.ends_with(';') {
            return false;
        }
    }

    false
}

const fn byte_is_identifier(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn closure_body(element: &str) -> &str {
    let unmoved = element.strip_prefix("move ").unwrap_or(element);

    let Some(parameters) = unmoved.strip_prefix('|') else {
        return element;
    };

    let body = parameters.split_once('|').map_or(element, |(_head, found)| found);

    assert!(body.len() <= element.len(), "a closure body is a part of its element");
    assert!(element.ends_with(body), "a closure body ends its element");

    body
}

fn code_of(line: &str) -> String {
    let mut code = String::with_capacity(line.len());
    let mut is_quoted = false;
    let mut is_escaped = false;

    for character in line.chars() {
        if is_escaped {
            is_escaped = false;

            continue;
        }

        is_escaped = is_quoted && character == '\\';

        if character == '"' {
            is_quoted = !is_quoted;

            continue;
        }

        if !is_quoted {
            code.push(character);
        }
    }

    let uncommented = code.split("//").next().unwrap_or_default().to_owned();

    assert!(uncommented.len() <= line.len(), "code is a part of its line");
    assert!(!uncommented.contains('"'), "no string survives into code");

    uncommented
}

fn code_quotes(line: &str) -> u64 {
    assert!(!line.contains('\n'), "a line holds no newline of its own");

    let mut quotes: u64 = 0;
    let mut is_escaped = false;

    for character in line.chars() {
        if is_escaped {
            is_escaped = false;

            continue;
        }

        is_escaped = character == '\\';

        if character == '"' {
            quotes = quotes.strict_add(1);
        }
    }

    assert!(quotes <= u64::try_from(line.len()).expect("a small line"), "a quote is a byte");

    quotes
}

fn element_commas(line: &str) -> u64 {
    let code = code_of(line);
    let trimmed = code.trim();
    let element = trimmed.strip_suffix(',').unwrap_or(trimmed);
    let mut angles: u64 = 0;
    let mut commas: u64 = 0;
    let mut depth: u64 = 0;
    let mut previous = [' ', ' '];

    for character in closure_body(element).chars() {
        let is_turbofish = character == '<' && previous == [':', ':'];

        match character {
            '(' | '[' | '{' => depth = depth.strict_add(1),
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            '<' if is_turbofish => {
                angles = angles.strict_add(1);
                depth = depth.strict_add(1);
            }
            '>' if angles > 0 => {
                angles = angles.strict_sub(1);
                depth = depth.saturating_sub(1);
            }
            ',' if depth == 0 => commas = commas.strict_add(1),
            _ => {}
        }

        previous = [previous[1], character];
    }

    assert!(commas <= u64::try_from(element.len()).expect("a small line"), "a comma is a byte");
    assert!(!element.ends_with(','), "the trailing comma is not counted");

    commas
}

#[test]
fn every_dependency_stays_inside_the_module_that_owns_it() {
    let sources = sources_library();
    let mut failures: Vec<String> = Vec::new();

    for (token, owners) in OWNERSHIP {
        assert!(!owners.is_empty(), "{token} has an owner");

        let reaching: Vec<&Source> =
            sources.iter().filter(|source| token_is_present(source, token)).collect();

        for source in &reaching {
            if !owners.contains(&source.name.as_str()) {
                failures.push(format!(
                    "{} reaches {token}, which only {} may",
                    source.name,
                    owners.join(" and "),
                ));
            }
        }

        for owner in owners {
            if !reaching.iter().any(|source| source.name == *owner) {
                failures.push(format!("{owner} no longer reaches {token}; drop it from the table"));
            }
        }
    }

    assert!(failures.is_empty(), "untidy:\n{}", failures.join("\n"));
}

#[test]
fn every_module_file_is_declared_and_every_declaration_has_a_file() {
    let directory = root().join("src");
    let declared = modules_declared(&directory.join("lib.rs"));
    let mut failures: Vec<String> = Vec::new();

    for source in sources_library() {
        let stem = source.name.trim_end_matches(".rs").to_owned();
        let is_entry = ENTRY_POINTS.contains(&source.name.as_str());
        let is_undeclared = !is_entry && !declared.contains(&stem);

        if is_undeclared {
            failures.push(format!("{} is never declared in lib.rs", source.name));
        }
    }

    for stem in &declared {
        let candidates =
            [directory.join(format!("{stem}.rs")), directory.join(stem).join("mod.rs")];

        if !candidates.iter().any(|path| path.is_file()) {
            failures.push(format!("lib.rs declares {stem}, which has no file"));
        }
    }

    assert!(declared.contains(&"client".to_owned()), "the root declares the client");
    assert!(failures.is_empty(), "untidy:\n{}", failures.join("\n"));
}

#[test]
fn every_source_breathes_where_the_whitespace_rules_say() {
    let sources = sources_everywhere();
    let mut failures: Vec<String> = Vec::new();

    for source in &sources {
        let lines: Vec<&str> = source.text.lines().collect();

        failures.extend(blank_failures(source, &lines));
        failures.extend(fence_failures(source, &lines));
        failures.extend(packed_failures(source, &lines));
    }

    assert!(!sources.is_empty(), "the whitespace rules have sources to read");
    assert!(failures.is_empty(), "untidy:\n{}", failures.join("\n"));
}

#[test]
fn every_source_ends_on_exactly_one_newline_with_no_trailing_space() {
    let sources = sources_everywhere();
    let mut failures: Vec<String> = Vec::new();

    for source in &sources {
        let is_misended = !source.text.ends_with('\n') || source.text.ends_with("\n\n");

        if is_misended {
            failures.push(format!("{} does not end on exactly one newline", source.path.display()));
        }

        if source.text.lines().any(|line| line.ends_with(' ')) {
            failures.push(format!("{} carries trailing whitespace", source.path.display()));
        }
    }

    assert!(!sources.is_empty(), "the file shape rules have sources to read");
    assert!(failures.is_empty(), "untidy:\n{}", failures.join("\n"));
}

#[test]
fn every_source_is_ascii_with_no_tab_or_carriage_return() {
    let mut failures: Vec<String> = Vec::new();

    let sources = sources_everywhere();

    for source in &sources {
        if !source.text.is_ascii() {
            failures.push(format!("{} carries a byte outside ASCII", source.path.display()));
        }

        if source.text.contains('\t') {
            failures.push(format!("{} carries a tab", source.path.display()));
        }

        if source.text.contains('\r') {
            failures.push(format!("{} carries a carriage return", source.path.display()));
        }
    }

    assert!(!sources.is_empty(), "the check has sources to read");
    assert!(failures.is_empty(), "untidy:\n{}", failures.join("\n"));
}

fn fence_failures(source: &Source, lines: &[&str]) -> Vec<String> {
    let inside = strings_inside(lines);
    let mut failures: Vec<String> = Vec::new();

    for block in blocks_find(lines, &inside) {
        let spans = spans_find(lines, block, &inside);

        for (above, below) in spans.iter().zip(spans.iter().skip(1)) {
            if below.first != above.last.strict_add(1) {
                continue;
            }

            if !span_pair_is_fenced(lines, Neighbours { above: *above, below: *below }) {
                continue;
            }

            failures.push(format!(
                "{}:{} lets a multi-line statement touch its neighbour",
                source.path.display(),
                below.first.strict_add(1),
            ));
        }
    }

    assert!(failures.len() <= lines.len(), "each line fails at most once");
    assert!(inside.len() <= lines.len(), "at most every line sits inside a string");

    failures
}

fn file_name(path: &Path) -> String {
    let name = path.file_name().and_then(|name| name.to_str()).expect("a source file name");

    assert!(!name.is_empty(), "a source file has a name");
    assert!(path.extension().is_some_and(|extension| extension == "rs"), "a source is Rust");

    name.to_owned()
}

fn function_indent(line: &str) -> Option<u64> {
    assert!(!line.contains('\n'), "a signature is read one line at a time");

    let trimmed = line.trim_start();

    let signature = trimmed
        .trim_start_matches("pub(crate) ")
        .trim_start_matches("pub ")
        .trim_start_matches("const ")
        .trim_start_matches("unsafe ")
        .trim_start_matches("extern ");

    if !signature.starts_with("fn ") {
        return None;
    }

    let indent = u64::try_from(line.len().strict_sub(trimmed.len())).ok();

    assert!(indent.is_some_and(|found| found.is_multiple_of(4)), "a function indents by levels");

    indent
}

fn function_line_count(lines: &[&str], signature: Signature) -> u64 {
    assert!(
        signature.start < u64::try_from(lines.len()).expect("a small file"),
        "a signature is in the file",
    );

    let closing = format!("{}}}", " ".repeat(signature.indent.try_into().expect("a small indent")));
    let skipped = lines.iter().skip(signature.start.try_into().expect("a small offset"));

    for (offset, line) in (0_u64..).zip(skipped) {
        if *line == closing {
            return offset.strict_add(1);
        }
    }

    0
}

fn indent_of(line: &str) -> u64 {
    let indent = line.len().strict_sub(line.trim_start_matches(' ').len());

    assert!(indent <= line.len(), "an indent is a prefix of its line");

    u64::try_from(indent).expect("a small indent")
}

fn line_at<'text>(lines: &[&'text str], index: Option<u64>) -> &'text str {
    assert!(
        index.is_none_or(|found| found <= u64::try_from(lines.len()).expect("a small file")),
        "a line is looked for at most one past the end",
    );

    index
        .and_then(|found| usize::try_from(found).ok())
        .and_then(|found| lines.get(found))
        .copied()
        .unwrap_or_default()
}

fn list_openers(lines: &[&str], inside: &[u64]) -> Vec<u64> {
    let mut openers: Vec<u64> = Vec::new();
    let mut region_indent: Option<u64> = None;

    for (index, line) in (0_u64..).zip(lines) {
        if line.trim().is_empty() {
            continue;
        }

        if inside.binary_search(&index).is_ok() {
            continue;
        }

        let indent = indent_of(line);

        if region_indent.is_some_and(|opened| indent <= opened) {
            region_indent = None;
        }

        let code = code_of(line);
        let opening = code.trim_end();
        let is_macro = MACRO_OPENERS.iter().any(|opener| opening.ends_with(opener));
        let is_open = opening.ends_with(['(', '[', '{']);

        match (region_indent, is_macro, is_open) {
            (None, true, _) => {
                openers.push(index);
                region_indent = Some(indent);
            }
            (Some(_), _, true) => openers.push(index),
            (None, false, _) | (Some(_), _, false) => {}
        }
    }

    assert!(openers.len() <= lines.len(), "each line opens at most one list");
    assert!(openers.is_sorted(), "openers are found in order");

    openers
}

fn modules_declared(lib_path: &Path) -> Vec<String> {
    let text = fs::read_to_string(lib_path).expect("read lib.rs");
    let mut declared: Vec<String> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim().trim_start_matches("pub ");

        let Some(stem) = trimmed.strip_prefix("mod ").and_then(|rest| rest.strip_suffix(';'))
        else {
            continue;
        };

        declared.push(stem.to_owned());
    }

    assert!(!declared.is_empty(), "the root declares modules");
    assert!(declared.iter().all(|stem| !stem.is_empty()), "each declared module has a name");

    declared
}

#[test]
fn no_function_outgrows_the_line_budget() {
    let mut failures: Vec<String> = Vec::new();

    let sources = sources_everywhere();

    for source in &sources {
        let lines: Vec<&str> = source.text.lines().collect();

        for (index, line) in (0_u64..).zip(&lines) {
            let Some(indent) = function_indent(line) else {
                continue;
            };

            if !body_is_present(&lines, index) {
                continue;
            }

            let line_count = function_line_count(&lines, Signature { indent, start: index });

            if line_count > FUNCTION_LINE_COUNT_MAX {
                failures.push(format!(
                    "{}:{} runs {line_count} lines",
                    source.path.display(),
                    index.strict_add(1),
                ));
            }
        }
    }

    assert!(!sources.is_empty(), "the check has sources to read");
    assert!(failures.is_empty(), "untidy:\n{}", failures.join("\n"));
}

#[test]
fn no_source_suppresses_a_lint_instead_of_fixing_it() {
    let markers = [
        ["#[", "allow("].concat(),
        ["#[", "expect("].concat(),
        ["#![", "allow("].concat(),
        ["#![", "expect("].concat(),
        ["tigerstyle", "-ignore"].concat(),
    ];

    let mut failures: Vec<String> = Vec::new();

    let sources = sources_everywhere();

    for source in &sources {
        for marker in &markers {
            if source.text.contains(marker.as_str()) {
                failures.push(format!("{} carries {marker}", source.path.display()));
            }
        }
    }

    assert!(!sources.is_empty(), "the check has sources to read");
    assert!(failures.is_empty(), "untidy:\n{}", failures.join("\n"));
}

fn packed_failures(source: &Source, lines: &[&str]) -> Vec<String> {
    let inside = strings_inside(lines);
    let mut failures: Vec<String> = Vec::new();

    for opener in list_openers(lines, &inside) {
        let element_indent = indent_of(line_at(lines, Some(opener))).strict_add(LIST_INDENT);
        let first = usize::try_from(opener.strict_add(1)).expect("a small file");

        for (index, line) in (0_u64..).zip(lines).skip(first) {
            if line.trim().is_empty() {
                continue;
            }

            if inside.binary_search(&index).is_ok() {
                continue;
            }

            let indent = indent_of(line);

            if indent < element_indent {
                break;
            }

            if indent > element_indent {
                continue;
            }

            if element_commas(line) > 0 {
                failures.push(format!(
                    "{}:{} packs two elements of a wrapped list onto one line",
                    source.path.display(),
                    index.strict_add(1),
                ));
            }
        }
    }

    assert!(failures.len() <= lines.len(), "each line fails at most once per list");
    assert!(inside.len() <= lines.len(), "at most every line sits inside a string");

    failures
}

fn root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();

    assert!(root.join("Cargo.toml").is_file(), "the manifest directory is the repository");
    assert!(root.join("src").is_dir(), "the repository holds its sources");

    root
}

fn source_read(path: PathBuf) -> Source {
    assert!(path.extension().is_some_and(|extension| extension == "rs"), "a source is Rust");

    let text = fs::read_to_string(&path).expect("read a source file");

    assert!(
        u64::try_from(text.len()).expect("a small file") <= FILE_BYTES_MAX,
        "a source file fits the tidy budget",
    );

    Source { name: file_name(&path), path, text }
}

fn sources_everywhere() -> Vec<Source> {
    let mut pending: Vec<PathBuf> = ROOTS.iter().map(|name| root().join(name)).collect();
    let mut sources: Vec<Source> = Vec::new();

    for _directory_index in 0..DIRECTORY_COUNT_MAX {
        let Some(directory) = pending.pop() else {
            break;
        };

        if !directory.is_dir() {
            continue;
        }

        for entry in fs::read_dir(&directory).expect("read a source directory") {
            let path = entry.expect("a directory entry").path();
            let is_rust = path.extension().is_some_and(|extension| extension == "rs");

            if path.is_dir() {
                pending.push(path);

                continue;
            }

            if is_rust {
                sources.push(source_read(path));
            }
        }
    }

    assert!(pending.is_empty(), "every source directory is walked inside the budget");
    assert!(!sources.is_empty(), "the repository holds sources");

    sources
}

fn sources_library() -> Vec<Source> {
    let directory = root().join("src");
    let mut sources: Vec<Source> = Vec::new();

    for entry in fs::read_dir(&directory).expect("read the source directory") {
        let path = entry.expect("a source directory entry").path();

        if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(source_read(path));
        }
    }

    sources.sort_by(|left, right| left.name.cmp(&right.name));

    assert!(!sources.is_empty(), "the library has sources");

    assert!(
        u32::try_from(sources.len()).expect("a small count") <= FILE_COUNT_MAX,
        "the library stays inside the file budget",
    );

    sources
}

fn span_pair_is_fenced(lines: &[&str], neighbours: Neighbours) -> bool {
    assert_eq!(neighbours.below.first, neighbours.above.last.strict_add(1), "neighbours touch");
    assert!(neighbours.above.first <= neighbours.above.last, "a statement spans its lines");

    let above = neighbours.above;
    let below = neighbours.below;
    let is_multi = above.last > above.first || below.last > below.first;
    let above_text = line_at(lines, Some(above.first)).trim_start();
    let below_text = line_at(lines, Some(below.first)).trim_start();
    let is_marked = above_text.starts_with(['#', '/']) || above_text.starts_with("use ");
    let is_listed = code_of(line_at(lines, Some(above.last))).trim_end().ends_with(',');
    let is_arm = code_of(above_text).contains("=>") || code_of(below_text).contains("=>");
    let is_import = below_text.starts_with("use ") || below_text.starts_with("pub use ");

    is_multi && !is_marked && !is_listed && !is_arm && !is_import
}

fn spans_find(lines: &[&str], block: Block, inside: &[u64]) -> Vec<Span> {
    assert!(block.start <= block.end, "a block opens before it closes");
    assert!(block.indent.is_multiple_of(4), "a block indents by four spaces a level");

    let mut spans: Vec<Span> = Vec::new();
    let mut index = block.start;

    while index < block.end {
        let line = line_at(lines, Some(index));

        let is_statement =
            !line.trim().is_empty() && !inside.contains(&index) && indent_of(line) == block.indent;

        if !is_statement {
            index = index.strict_add(1);

            continue;
        }

        let first = index;

        index = index.strict_add(1);

        while index < block.end {
            if !statement_continues(line_at(lines, Some(index)), block.indent) {
                break;
            }

            index = index.strict_add(1);
        }

        spans.push(Span { first, last: index.strict_sub(1) });
    }

    assert!(spans.iter().all(|span| span.first <= span.last), "a statement spans its own lines");

    spans
}

fn statement_continues(line: &str, indent: u64) -> bool {
    assert!(indent.is_multiple_of(4), "a statement indents by four spaces a level");

    let body = line.trim_start();
    let is_deeper = indent_of(line) > indent;
    let is_tail = indent_of(line) == indent && body.starts_with([')', ']', '}', '.', '?', '{']);

    let is_clause = indent_of(line) == indent
        && (body.starts_with("else") || body.starts_with("where") || body.starts_with("->"));

    !body.is_empty() && (is_deeper || is_tail || is_clause)
}

fn strings_inside(lines: &[&str]) -> Vec<u64> {
    let mut inside: Vec<u64> = Vec::new();
    let mut is_open = false;

    for (index, line) in (0_u64..).zip(lines) {
        if is_open {
            inside.push(index);
        }

        let quotes = code_quotes(line);

        if !quotes.is_multiple_of(2) {
            is_open = !is_open;
        }
    }

    assert!(inside.len() <= lines.len(), "at most every line sits inside a string");
    assert!(inside.is_sorted(), "string lines are found in order");

    inside
}

#[test]
fn the_packed_list_rule_catches_what_it_is_written_for() {
    let source = Source {
        name: "probe.rs".to_owned(),
        path: PathBuf::from("probe.rs"),
        text: String::new(),
    };

    let packed =
        ["fn probe() {", "    assert_eq!(", "        a, b,", "        \"same\",", "    );", "}"];

    let object = ["fn probe() {", "    json!({", "        \"a\": 1, \"b\": 2,", "    })", "}"];

    let nested = [
        "fn probe() {",
        "    json!({",
        "        \"issues\": [",
        "            1, 2,",
        "        ],",
        "    })",
        "}",
    ];

    let wrapped = [
        "fn probe() {",
        "    assert_eq!(",
        "        a,",
        "        f(b, c),",
        "        |left, right| left.cmp(right),",
        "        HashMap::<String, u64>::new(),",
        "        \"same, and more\",",
        "    );",
        "}",
    ];

    assert_eq!(packed_failures(&source, &packed).len(), 1, "two arguments on one line fail");
    assert_eq!(packed_failures(&source, &object).len(), 1, "two entries on one line fail");
    assert_eq!(packed_failures(&source, &nested).len(), 1, "a list nested in a macro is held too");
    assert!(packed_failures(&source, &wrapped).is_empty(), "one argument a line passes");
}

#[test]
fn the_whitespace_rules_catch_what_they_are_written_for() {
    let source = Source {
        name: "probe.rs".to_owned(),
        path: PathBuf::from("probe.rs"),
        text: String::new(),
    };

    let opened = ["fn probe() {", "", "    let a = 1;", "}"];
    let doubled = ["fn probe() {", "    let a = 1;", "", "", "    a", "}"];
    let touching = ["fn probe() {", "    let a = 1;", "    if a > 0 {", "        a", "    }", "}"];

    let fenced =
        ["fn probe() {", "    let a = 1;", "", "    if a > 0 {", "        a", "    }", "}"];

    assert_eq!(blank_failures(&source, &opened).len(), 1, "a blank line opening a block fails");
    assert_eq!(blank_failures(&source, &doubled).len(), 1, "a doubled blank line fails");
    assert_eq!(fence_failures(&source, &touching).len(), 1, "a block touching a statement fails");
    assert!(fence_failures(&source, &fenced).is_empty(), "a fenced block passes");
    assert!(blank_failures(&source, &fenced).is_empty(), "a well spaced function passes");
}

fn token_is_present(source: &Source, token: &str) -> bool {
    assert!(!token.is_empty(), "a token is something to look for");
    assert!(!source.name.is_empty(), "a token is looked for in a named source");

    let text = source.text.as_str();
    let bytes = text.as_bytes();
    let mut from = 0;

    for _match_index in 0..MATCH_COUNT_MAX {
        let Some(offset) = text.get(from..).and_then(|rest| rest.find(token)) else {
            return false;
        };

        let at = from.strict_add(offset);
        let is_bounded = at == 0 || !byte_is_identifier(bytes[at.strict_sub(1)]);

        if is_bounded {
            return true;
        }

        from = at.strict_add(1);
    }

    false
}
