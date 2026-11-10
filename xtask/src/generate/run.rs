use std::fs;
use std::path::Path;

use serde_json::Value;

use super::emit;
use super::model;
use super::overrides;
use super::product::PRODUCTS;
use super::sweep;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Roots<'roots> {
    pub source: &'roots Path,
    pub sweep: &'roots Path,
    pub target: &'roots Path,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Tally {
    pub file_count: u64,
    pub operation_count: u64,
    pub schema_count: u64,
}

pub(crate) fn count_of<Item>(items: &[Item]) -> u64 {
    let count = u64::try_from(items.len()).unwrap_or(u64::MAX);

    assert!(count < u64::MAX, "a count fits its width");
    assert_eq!(count == 0, items.is_empty(), "a count is zero exactly when the slice is empty");

    count
}

fn directory_clear(directory: &Path) -> Result<(), String> {
    assert!(directory.is_absolute(), "a generated directory is named in full");

    if directory.is_dir() {
        fs::remove_dir_all(directory)
            .map_err(|cause| format!("cannot clear {}: {cause}", directory.display()))?;
    }

    assert!(!directory.exists(), "a cleared directory is gone before it is written again");

    Ok(())
}

fn document_read(source: &Path, module: &str) -> Result<Value, String> {
    assert!(!module.is_empty(), "a specification is named for its module");
    assert!(source.is_dir(), "the generator reads the workspace root");

    let path = source.join("spec").join(format!("{module}.json"));

    let text = fs::read_to_string(&path)
        .map_err(|cause| format!("cannot read {}: {cause}", path.display()))?;

    serde_json::from_str(&text).map_err(|cause| format!("cannot parse {}: {cause}", path.display()))
}

pub(crate) fn run(roots: Roots<'_>) -> Result<Tally, String> {
    assert!(roots.source.is_dir(), "the generator reads the workspace root");

    let mut tally = Tally { file_count: 0, operation_count: 0, schema_count: 0 };
    let mut held = overrides::Held::default();

    directory_clear(roots.sweep)?;

    for product in &PRODUCTS {
        let mut document = document_read(roots.source, product.module)?;
        let edit_count = overrides::edit(product.module, &mut document)?;
        let built = model::model_build(product, &document)?;

        held.edit_count = held.edit_count.strict_add(edit_count);

        directory_clear(&roots.target.join(product.module))?;

        let schema_count = u64::try_from(built.schemas.len()).unwrap_or(u64::MAX);

        tally.operation_count = tally.operation_count.strict_add(count_of(&built.operations));
        tally.schema_count = tally.schema_count.strict_add(schema_count);

        for file in emit::files(product, &built) {
            write_file(&roots.target.join(&file.path), &file.lines)?;

            tally.file_count = tally.file_count.strict_add(1);
        }

        let swept = sweep::file(product, &built);

        write_file(&roots.sweep.join(&swept.path), &swept.lines)?;

        tally.file_count = tally.file_count.strict_add(1);

        for (schema, members) in &built.schemas {
            let wires = members.iter().map(|member| (schema.clone(), member.wire.clone()));

            held.fields.extend(wires);
        }

        for operation in &built.operations {
            let identifier = &operation.identifier;
            let wires = operation.parameters.iter().map(|parameter| &parameter.wire);

            held.parameters.extend(wires.map(|wire| (identifier.clone(), wire.clone())));
        }

        held.operations.extend(built.identifiers);
    }

    overrides::check(&held)?;

    let root = sweep::root();

    write_file(&roots.sweep.join(&root.path), &root.lines)?;

    assert!(tally.operation_count >= 1, "the generator wrote operations");
    assert!(tally.file_count >= tally.operation_count, "each operation holds a file");

    Ok(tally)
}

fn write_file(path: &Path, lines: &[String]) -> Result<(), String> {
    assert!(!lines.is_empty(), "a generated file holds lines");

    let directory = path.parent().ok_or("a generated file has a directory")?;

    fs::create_dir_all(directory)
        .map_err(|cause| format!("cannot create {}: {cause}", directory.display()))?;

    let mut text = lines.join("\n");

    text.push('\n');

    assert!(text.ends_with('\n'), "a generated file ends its last line");

    fs::write(path, text).map_err(|cause| format!("cannot write {}: {cause}", path.display()))
}
