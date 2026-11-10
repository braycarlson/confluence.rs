use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::generate::{self, Roots};

const EXAMPLES: [&str; 1] = ["offline"];
const GENERATED_ROOTS: [&str; 3] = ["automation", "v1", "v2"];
const GENERATED_SCRATCH: &str = "target/generated";
const GENERATED_SWEEP: [&str; 3] = ["tests", "it", "sweep"];
const RUSTFMT_BATCH_BYTES_MAX: u32 = 16_000;

const SPECIFICATIONS: [(&str, &str); 3] = [
    ("automation", "https://developer.atlassian.com/cloud/automation/swagger.v3.json"),
    ("v1", "https://developer.atlassian.com/cloud/confluence/swagger.v3.json"),
    ("v2", "https://dac-static.atlassian.com/cloud/confluence/openapi-v2.v3.json"),
];

const TIGERSTYLE_ROOTS: [&str; 4] = ["src", "tests", "examples", "xtask"];
const WALK_COUNT_MAX: u32 = 8_192;

#[derive(Debug)]
struct Pair {
    expected: PathBuf,
    found: PathBuf,
}

fn cargo() -> String {
    let program = env::var("CARGO").unwrap_or_else(|_absent| "cargo".to_owned());

    assert!(!program.is_empty(), "a cargo program is named");
    assert!(!program.contains('\n'), "a program name is one line");

    program
}

pub(crate) fn checks_run() -> Result<(), String> {
    let root = workspace_root()?;

    assert!(root.join("Cargo.toml").is_file(), "ci runs at the workspace root");
    assert!(root.join("spec").is_dir(), "ci reads the pinned specifications");

    drift_step(&root)?;
    format_step(&root)?;
    step(&root, "clippy", &["clippy", "--workspace", "--all-targets"])?;
    step(&root, "test", &["test", "--workspace"])?;
    step(&root, "fuzz", &["run", "--release", "--bin", "fuzz", "--", "smoke"])?;

    for example in EXAMPLES {
        step(&root, example, &["run", "--quiet", "--example", example])?;
    }

    tigerstyle_step(&root)?;

    report("ci: green")
}

fn drift_step(root: &Path) -> Result<(), String> {
    assert!(root.is_absolute(), "the drift check runs at the workspace root");

    report("ci: generated")?;

    let scratch = root.join(GENERATED_SCRATCH);
    let sweep = scratch.join("sweep");
    let roots = Roots { source: root, sweep: &sweep, target: &scratch };
    let tally = generate::run(roots)?;

    assert!(tally.file_count >= 1, "the generator wrote files");

    rustfmt_tree(roots)?;

    let mut differences: Vec<String> = Vec::new();

    for module in GENERATED_ROOTS {
        let pair = Pair { expected: scratch.join(module), found: root.join("src").join(module) };

        differences.extend(tree_compare(pair)?);
    }

    differences.extend(tree_compare(Pair { expected: sweep, found: sweep_directory(root) })?);

    if !differences.is_empty() {
        return Err(format!("run cargo xtask generate: {}", differences.join(", ")));
    }

    assert!(differences.is_empty(), "the generated modules match the specifications");

    Ok(())
}

fn format_step(root: &Path) -> Result<(), String> {
    assert!(root.is_absolute(), "rustfmt runs at the workspace root");

    report("ci: fmt")?;

    let status = Command::new("rustup")
        .args(["run", "nightly", "cargo", "fmt", "--all", "--check"])
        .current_dir(root)
        .status()
        .map_err(|cause| format!("cannot run nightly rustfmt through rustup: {cause}"))?;

    if !status.success() {
        return Err(format!("fmt failed: {status}"));
    }

    assert!(status.success(), "a passing format check leaves nothing to rewrite");

    Ok(())
}

pub(crate) fn fuzz(arguments: &[String]) -> Result<(), String> {
    assert!(arguments.len() <= 3, "a fuzz run takes a name, a seed, and a case count at most");

    let root = workspace_root()?;
    let mut forwarded = vec!["run", "--release", "--bin", "fuzz", "--"];

    forwarded.extend(arguments.iter().map(String::as_str));

    assert!(forwarded.len() >= 5, "a fuzz run always reaches the fuzz binary");

    step(&root, "fuzz", &forwarded)
}

pub(crate) fn generated() -> Result<(), String> {
    let root = workspace_root()?;
    let target = root.join("src");
    let sweep = sweep_directory(&root);
    let roots = Roots { source: &root, sweep: &sweep, target: &target };
    let tally = generate::run(roots)?;

    assert!(tally.operation_count >= 1, "the generator wrote operations");

    rustfmt_tree(roots)?;

    let counts = format!("{} operations and {} schemas", tally.operation_count, tally.schema_count);

    report(&format!("generated {counts} in {} files", tally.file_count))
}

fn report(text: &str) -> Result<(), String> {
    assert!(!text.is_empty(), "a report says something");
    assert!(!text.ends_with('\n'), "a report carries its own newline");

    writeln!(io::stdout().lock(), "{text}")
        .map_err(|cause| format!("cannot write to standard output: {cause}"))
}

fn rustfmt_batches(files: &[PathBuf]) -> Vec<Vec<&Path>> {
    assert!(!files.is_empty(), "a batch plan needs at least one source");

    let mut batches: Vec<Vec<&Path>> = Vec::new();
    let mut batch: Vec<&Path> = Vec::new();
    let mut batch_bytes: u32 = 0;

    for file in files {
        let length = u32::try_from(file.as_os_str().len()).unwrap_or(u32::MAX).saturating_add(1);

        assert!(length <= RUSTFMT_BATCH_BYTES_MAX, "a single source path fits in one batch");

        if batch_bytes.saturating_add(length) > RUSTFMT_BATCH_BYTES_MAX {
            batches.push(batch);
            batch = Vec::new();
            batch_bytes = 0;
        }

        batch.push(file.as_path());
        batch_bytes = batch_bytes.saturating_add(length);
    }

    batches.push(batch);

    let planned = batches.iter().map(Vec::len).sum::<usize>();

    assert!(batches.iter().all(|grouped| !grouped.is_empty()), "each batch formats a source");
    assert_eq!(planned, files.len(), "each source lands in exactly one batch");

    batches
}

fn rustfmt_tree(roots: Roots<'_>) -> Result<(), String> {
    assert!(roots.target.is_dir(), "rustfmt formats a generated tree");
    assert!(roots.sweep.is_dir(), "rustfmt formats the generated sweep");

    let root = workspace_root()?;
    let mut files: Vec<PathBuf> = tree_sources(roots.sweep)?;

    assert!(root.join("rustfmt.toml").is_file(), "rustfmt reads the workspace configuration");

    for module in GENERATED_ROOTS {
        files.extend(tree_sources(&roots.target.join(module))?);
    }

    assert!(!files.is_empty(), "a generated tree holds sources");

    for batch in rustfmt_batches(&files) {
        let status = Command::new("rustup")
            .args(["run", "nightly", "rustfmt", "--edition", "2024", "--config-path"])
            .arg(&root)
            .args(["--config", "skip_children=true"])
            .args(batch)
            .current_dir(&root)
            .status()
            .map_err(|cause| format!("cannot run nightly rustfmt through rustup: {cause}"))?;

        if !status.success() {
            return Err(format!("rustfmt failed: {status}"));
        }
    }

    Ok(())
}

pub(crate) fn specification() -> Result<(), String> {
    let root = workspace_root()?;

    assert!(root.join("spec").is_dir(), "the specifications are pinned in the repository");
    assert!(!SPECIFICATIONS.is_empty(), "at least one specification is pinned");

    for (module, url) in SPECIFICATIONS {
        let path = root.join("spec").join(format!("{module}.json"));

        let status = Command::new("curl")
            .args(["--fail", "--silent", "--show-error", "--location", "--output"])
            .arg(&path)
            .arg(url)
            .status()
            .map_err(|cause| format!("cannot run curl for {url}: {cause}"))?;

        if !status.success() {
            return Err(format!("downloading {url} failed: {status}"));
        }

        report(&format!("pinned {}", path.display()))?;
    }

    Ok(())
}

fn step(root: &Path, name: &str, arguments: &[&str]) -> Result<(), String> {
    assert!(!name.is_empty(), "a step has a name");
    assert!(!arguments.is_empty(), "a step runs something");

    report(&format!("ci: {name}"))?;

    let status = Command::new(cargo())
        .args(arguments)
        .current_dir(root)
        .status()
        .map_err(|cause| format!("cannot run {name}: {cause}"))?;

    if !status.success() {
        return Err(format!("{name} failed: {status}"));
    }

    Ok(())
}

fn sweep_directory(root: &Path) -> PathBuf {
    assert!(root.is_absolute(), "the sweep sits under the workspace root");

    let directory = GENERATED_SWEEP.iter().fold(root.to_path_buf(), |path, piece| path.join(piece));

    assert!(directory.starts_with(root), "the sweep stays inside the workspace");

    directory
}

fn tigerstyle_step(root: &Path) -> Result<(), String> {
    assert!(root.is_absolute(), "tigerstyle runs at the workspace root");

    if !env::var("CI").unwrap_or_default().is_empty() {
        return report("ci: tigerstyle skipped; the checker is not installed on CI runners");
    }

    report("ci: tigerstyle")?;

    let status = Command::new("tigerstyle")
        .arg("check")
        .args(TIGERSTYLE_ROOTS)
        .current_dir(root)
        .status()
        .map_err(|cause| format!("cannot run tigerstyle, which ci needs locally: {cause}"))?;

    if !status.success() {
        return Err(format!("tigerstyle failed: {status}"));
    }

    Ok(())
}

fn tree_compare(pair: Pair) -> Result<Vec<String>, String> {
    assert!(pair.expected.is_dir(), "the scratch tree was just generated");
    assert!(pair.found.is_dir(), "the checked in tree exists");

    let mut pending: Vec<Pair> = vec![pair];
    let mut differences: Vec<String> = Vec::new();

    for _walk_index in 0..WALK_COUNT_MAX {
        let Some(current) = pending.pop() else {
            break;
        };

        for name in tree_names(&current)? {
            let expected = current.expected.join(&name);
            let found = current.found.join(&name);

            if expected.is_dir() {
                pending.push(Pair { expected, found });

                continue;
            }

            if fs::read(&expected).ok() != fs::read(&found).ok() {
                differences.push(found.display().to_string());
            }
        }
    }

    assert!(pending.is_empty(), "the comparison reached every directory");

    Ok(differences)
}

fn tree_names(pair: &Pair) -> Result<Vec<OsString>, String> {
    assert!(pair.expected.is_dir(), "the scratch tree was just generated");
    assert!(pair.found.is_dir(), "the checked in tree holds this directory");

    let mut names: Vec<OsString> = Vec::new();

    for directory in [&pair.expected, &pair.found] {
        let entries = fs::read_dir(directory)
            .map_err(|cause| format!("cannot list {}: {cause}", directory.display()))?;

        for found in entries {
            names.push(found.map_err(|cause| format!("cannot list a file: {cause}"))?.file_name());
        }
    }

    names.sort();
    names.dedup();

    Ok(names)
}

fn tree_sources(directory: &Path) -> Result<Vec<PathBuf>, String> {
    assert!(directory.is_dir(), "a generated module is a directory");

    let mut pending: Vec<PathBuf> = vec![directory.to_path_buf()];
    let mut sources: Vec<PathBuf> = Vec::new();

    for _walk_index in 0..WALK_COUNT_MAX {
        let Some(current) = pending.pop() else {
            break;
        };

        let entries = fs::read_dir(&current)
            .map_err(|cause| format!("cannot list {}: {cause}", current.display()))?;

        for found in entries {
            let path = found.map_err(|cause| format!("cannot list a file: {cause}"))?.path();

            if path.is_dir() {
                pending.push(path);

                continue;
            }

            if !path.ends_with("mod.rs") {
                sources.push(path);
            }
        }
    }

    assert!(pending.is_empty(), "the walk reached every directory");

    Ok(sources)
}

fn workspace_root() -> Result<PathBuf, String> {
    let manifest_directory = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_directory.parent().ok_or("the xtask manifest directory has no parent")?;

    assert!(root.join("Cargo.toml").is_file(), "the workspace root holds its manifest");
    assert!(root.join("src").is_dir(), "the workspace root holds the sources");

    Ok(root.to_path_buf())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::{RUSTFMT_BATCH_BYTES_MAX, rustfmt_batches};

    #[test]
    fn a_batch_closes_only_when_the_next_source_would_overflow_it() {
        let files = sources(2_400);
        let batches = rustfmt_batches(&files);

        for (index, batch) in batches.iter().enumerate() {
            let bytes = argument_bytes(batch);

            assert!(bytes <= RUSTFMT_BATCH_BYTES_MAX, "a batch fits the argument budget");

            if let Some(following) = batches.get(index.saturating_add(1)) {
                let first = following.first().expect("a planned batch holds a source");
                let grown = bytes.saturating_add(argument_length(first));

                assert!(grown > RUSTFMT_BATCH_BYTES_MAX, "a batch takes each source that fits");
            }
        }
    }

    #[test]
    fn a_batch_plan_holds_each_source_once_and_in_order() {
        let files = sources(2_400);
        let batches = rustfmt_batches(&files);
        let flattened: Vec<&Path> = batches.iter().flatten().copied().collect();
        let expected: Vec<&Path> = files.iter().map(PathBuf::as_path).collect();

        assert!(batches.len() > 1, "a tree too long for one command line splits");
        assert_eq!(flattened, expected, "the batches hold each source once, in order");
    }

    #[test]
    fn a_short_tree_stays_in_one_batch() {
        let files = sources(3);
        let batches = rustfmt_batches(&files);

        assert_eq!(batches.len(), 1, "a tree that fits one command line stays whole");
        assert_eq!(batches.first().map(Vec::len), Some(3), "the batch holds each source");
    }

    fn argument_bytes(batch: &[&Path]) -> u32 {
        assert!(!batch.is_empty(), "a measured batch holds a source");

        batch.iter().map(|path| argument_length(path)).fold(0, u32::saturating_add)
    }

    fn argument_length(path: &Path) -> u32 {
        assert!(!path.as_os_str().is_empty(), "a measured source has a path");

        u32::try_from(path.as_os_str().len()).unwrap_or(u32::MAX).saturating_add(1)
    }

    fn sources(count: u32) -> Vec<PathBuf> {
        assert!(count >= 1, "a test tree holds at least one source");

        (0..count)
            .map(|index| {
                PathBuf::from(format!(
                    "target/generated/v2/operation/{index:05}_get_inline_comment_children_by_id.rs"
                ))
            })
            .collect()
    }
}
