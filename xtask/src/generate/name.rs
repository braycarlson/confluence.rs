use core::mem;

const ABBREVIATIONS: [(&str, &str); 91] = [
    ("abs", "absolute"),
    ("acc", "accumulator"),
    ("addr", "address"),
    ("arg", "argument"),
    ("args", "arguments"),
    ("attr", "attribute"),
    ("attrs", "attributes"),
    ("auth", "authentication"),
    ("btn", "button"),
    ("buf", "buffer"),
    ("bufs", "buffers"),
    ("calc", "calculate"),
    ("cb", "callback"),
    ("cfg", "configuration"),
    ("chan", "channel"),
    ("cnt", "count"),
    ("col", "column"),
    ("cols", "columns"),
    ("conf", "configuration"),
    ("ctx", "context"),
    ("cur", "cursor"),
    ("curr", "current"),
    ("desc", "description"),
    ("dest", "destination"),
    ("dict", "dictionary"),
    ("dir", "directory"),
    ("dst", "destination"),
    ("elem", "element"),
    ("elems", "elements"),
    ("env", "environment"),
    ("err", "error"),
    ("errs", "errors"),
    ("evt", "event"),
    ("expr", "expression"),
    ("ext", "extension"),
    ("fmt", "format"),
    ("fns", "functions"),
    ("gen", "generate"),
    ("hdr", "header"),
    ("idx", "index"),
    ("img", "image"),
    ("impl", "implementation"),
    ("info", "information"),
    ("init", "initialize"),
    ("inst", "instance"),
    ("iter", "iterator"),
    ("len", "length"),
    ("lst", "list"),
    ("mem", "memory"),
    ("mgr", "manager"),
    ("mod", "module"),
    ("msg", "message"),
    ("msgs", "messages"),
    ("num", "number"),
    ("obj", "object"),
    ("objs", "objects"),
    ("opt", "option"),
    ("opts", "options"),
    ("param", "parameter"),
    ("params", "parameters"),
    ("pkg", "package"),
    ("pos", "position"),
    ("prev", "previous"),
    ("proc", "process"),
    ("prop", "property"),
    ("props", "properties"),
    ("ptr", "pointer"),
    ("qty", "quantity"),
    ("recv", "receive"),
    ("ref", "reference"),
    ("refs", "references"),
    ("req", "request"),
    ("reqs", "requests"),
    ("res", "result"),
    ("resp", "response"),
    ("ret", "returned"),
    ("sep", "separator"),
    ("seq", "sequence"),
    ("sig", "signature"),
    ("spec", "specification"),
    ("src", "source"),
    ("str", "string"),
    ("sync", "synchronize"),
    ("tbl", "table"),
    ("tmp", "temporary"),
    ("txt", "text"),
    ("typ", "type"),
    ("usr", "user"),
    ("val", "value"),
    ("vals", "values"),
    ("var", "variable"),
];

const ACRONYMS: [&str; 62] = [
    "abi",
    "ack",
    "adf",
    "aes",
    "api",
    "ari",
    "ascii",
    "cli",
    "cpu",
    "cql",
    "crc",
    "css",
    "csv",
    "db",
    "dns",
    "eof",
    "fd",
    "ffi",
    "gc",
    "gpu",
    "html",
    "http",
    "https",
    "id",
    "io",
    "ip",
    "ipc",
    "jit",
    "json",
    "jwt",
    "lru",
    "lsm",
    "mac",
    "md5",
    "nak",
    "os",
    "pid",
    "ram",
    "rpc",
    "rsa",
    "sdk",
    "sha",
    "sql",
    "ssh",
    "ssl",
    "tcp",
    "tls",
    "toml",
    "ttl",
    "udp",
    "ui",
    "uri",
    "url",
    "utf",
    "uuid",
    "vsr",
    "wal",
    "xml",
    "yaml",
    "aris",
    "ids",
    "urls",
];

const KEYWORDS: [&str; 52] = [
    "abstract",
    "as",
    "async",
    "await",
    "become",
    "box",
    "break",
    "const",
    "continue",
    "crate",
    "do",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "final",
    "fn",
    "for",
    "gen",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "macro",
    "match",
    "mod",
    "move",
    "mut",
    "override",
    "priv",
    "pub",
    "ref",
    "return",
    "self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "try",
    "type",
    "typeof",
    "unsafe",
    "unsized",
    "use",
    "virtual",
    "where",
    "while",
    "yield",
    "union",
];

const QUALIFIERS_BOUND: [&str; 19] = [
    "average",
    "avg",
    "cur",
    "current",
    "first",
    "initial",
    "last",
    "max",
    "maximum",
    "mean",
    "min",
    "minimum",
    "new",
    "next",
    "num",
    "old",
    "previous",
    "sum",
    "total",
];

const QUALIFIERS_MEASURE: [&str; 7] = ["count", "index", "len", "length", "n", "offset", "size"];
const QUALIFIERS_UNIT: [&str; 4] = ["bytes", "ms", "ns", "us"];
const TIER_BOUND: u32 = 3;
const TIER_MEASURE: u32 = 1;
const TIER_SUBJECT: u32 = 0;
const TIER_UNIT: u32 = 2;

#[derive(Clone, Copy, Debug)]
struct Neighbours {
    character: char,
    following: Option<char>,
    previous: Option<char>,
}

fn acronym_of(word: &str) -> Option<String> {
    assert!(!word.is_empty(), "a word holds at least one letter");

    let lower = word.to_ascii_lowercase();
    let stem = lower.trim_end_matches(|character: char| character.is_ascii_digit());

    if !ACRONYMS.contains(&stem) {
        return None;
    }

    let spelled = match stem {
        "aris" => format!("ARIs{}", lower.trim_start_matches("aris")),
        "ids" => format!("IDs{}", lower.trim_start_matches("ids")),
        "urls" => format!("URLs{}", lower.trim_start_matches("urls")),
        _ => lower.to_ascii_uppercase(),
    };

    assert_eq!(spelled.len(), word.len(), "an acronym keeps its length");

    Some(spelled)
}

fn big_endian(words: Vec<String>) -> Vec<String> {
    assert!(!words.is_empty(), "a name holds at least one word");

    let word_count = words.len();

    let mut ranked: Vec<(u32, String)> =
        words.into_iter().map(|word| (tier_of(&word), word)).collect();

    ranked.sort_by_key(|(tier, _word)| *tier);

    let ordered: Vec<String> = ranked.into_iter().map(|(_tier, word)| word).collect();

    assert_eq!(ordered.len(), word_count, "a reorder keeps every word");

    ordered
}

#[must_use]
pub(crate) fn constant_name(wire: &str) -> String {
    assert!(!wire.is_empty(), "a member is named");

    let lowered = big_endian(words_expanded(wire)).join("_");
    let named = name_spellable(lowered).to_ascii_uppercase();

    assert!(!named.is_empty(), "a constant has a name");
    assert_eq!(named.to_ascii_uppercase(), named, "a constant shouts");

    named
}

fn expanded(word: &str) -> String {
    assert!(!word.is_empty(), "a word holds at least one letter");

    let found = ABBREVIATIONS.iter().find(|(short, _long)| *short == word);
    let spelled = found.map_or(word, |(_short, long)| *long).to_owned();

    assert!(spelled.len() >= word.len(), "a word never shortens when it is spelled out");

    spelled
}

#[must_use]
pub(crate) fn field_name(wire: &str) -> String {
    assert!(!wire.is_empty(), "a parameter is named");

    let name = big_endian(words_expanded(wire)).join("_");

    if name == "type" {
        return "kind".to_owned();
    }

    if KEYWORDS.contains(&name.as_str()) {
        return format!("{name}_value");
    }

    let named = name_spellable(name);

    assert!(!named.is_empty(), "a field has a name");
    assert_eq!(named.to_ascii_lowercase(), named, "a field is lowercase");

    named
}

#[must_use]
pub(crate) fn function_name(type_name: &str) -> String {
    assert!(!type_name.is_empty(), "a type is named");

    let spelled: Vec<String> =
        words(type_name).into_iter().map(|word| expanded(&word.to_ascii_lowercase())).collect();

    let name = spelled.join("_");

    assert!(name.len() >= module_name(type_name).len(), "spelling a word out never shortens it");

    if KEYWORDS.contains(&name.as_str()) {
        return format!("{name}_operation");
    }

    name
}

#[must_use]
pub(crate) fn module_name(type_name: &str) -> String {
    assert!(!type_name.is_empty(), "a type is named");

    let lowered: Vec<String> =
        words(type_name).into_iter().map(|word| word.to_ascii_lowercase()).collect();

    let name = lowered.join("_");

    assert!(!name.is_empty(), "a module has a name");

    if KEYWORDS.contains(&name.as_str()) {
        return format!("{name}_operation");
    }

    assert!(!name.contains(' '), "a module name is one identifier");

    name
}

fn name_spellable(name: String) -> String {
    assert!(!name.is_empty(), "a name holds at least one word");

    let leads_with_digit = name.chars().next().is_some_and(|character| character.is_ascii_digit());

    if leads_with_digit {
        return format!("value_{name}");
    }

    if name.len() > 2 {
        return name;
    }

    if ACRONYMS.contains(&name.as_str()) {
        return name;
    }

    format!("{name}_value")
}

fn tier_of(word: &str) -> u32 {
    assert!(!word.is_empty(), "a word holds at least one letter");

    if QUALIFIERS_MEASURE.contains(&word) {
        return TIER_MEASURE;
    }

    if QUALIFIERS_UNIT.contains(&word) {
        return TIER_UNIT;
    }

    if QUALIFIERS_BOUND.contains(&word) {
        return TIER_BOUND;
    }

    assert_eq!(word.to_ascii_lowercase(), word, "a tier is read from a lowered word");

    TIER_SUBJECT
}

#[must_use]
pub(crate) fn type_name(wire: &str) -> String {
    assert!(!wire.is_empty(), "a type is named");

    let mut name = String::with_capacity(wire.len());

    for word in words(wire) {
        match acronym_of(&word) {
            Some(acronym) => name.push_str(&acronym),
            None => name.push_str(&word_title(&word)),
        }
    }

    let leads_with_digit = name.chars().next().is_some_and(|character| character.is_ascii_digit());

    let named = if leads_with_digit { format!("Value{name}") } else { name };

    assert!(!named.is_empty(), "a type has a name");

    named
}

fn word_ends_before(neighbours: Neighbours) -> bool {
    let Some(behind) = neighbours.previous else {
        return false;
    };

    if !neighbours.character.is_ascii_uppercase() {
        return false;
    }

    if behind.is_ascii_lowercase() {
        return true;
    }

    if behind.is_ascii_digit() {
        return true;
    }

    if !behind.is_ascii_uppercase() {
        return false;
    }

    assert!(behind.is_ascii_uppercase(), "a run of capitals ends before its last capital");
    assert!(neighbours.character.is_ascii_uppercase(), "a word starts on a capital");

    neighbours.following.is_some_and(|ahead| ahead.is_ascii_lowercase())
}

fn word_title(word: &str) -> String {
    assert!(!word.is_empty(), "a word holds at least one letter");

    let mut characters = word.chars();
    let first = characters.next().map(|character| character.to_ascii_uppercase());
    let title: String = first.into_iter().chain(characters).collect();

    assert_eq!(title.len(), word.len(), "a title keeps its length");

    title
}

fn words(wire: &str) -> Vec<String> {
    let plain = wire
        .replace("ARIs", "Aris")
        .replace("IDs", "Ids")
        .replace("OAuth", "Oauth")
        .replace("URLs", "Urls");

    let characters: Vec<char> = plain.chars().collect();
    let mut found: Vec<String> = Vec::new();
    let mut word = String::new();

    for (index, character) in characters.iter().enumerate() {
        if !character.is_ascii_alphanumeric() {
            if !word.is_empty() {
                found.push(mem::take(&mut word));
            }

            continue;
        }

        let previous = index.checked_sub(1).and_then(|position| characters.get(position));
        let following = characters.get(index.strict_add(1));

        let neighbours = Neighbours {
            character: *character,
            following: following.copied(),
            previous: previous.copied(),
        };

        if word_ends_before(neighbours) {
            found.push(mem::take(&mut word));
        }

        word.push(*character);
    }

    if !word.is_empty() {
        found.push(word);
    }

    assert!(!found.is_empty(), "a name holds at least one word");
    assert!(found.iter().all(|piece| !piece.is_empty()), "a word is never empty");

    found
}

fn words_expanded(wire: &str) -> Vec<String> {
    let found: Vec<String> =
        words(wire).iter().map(|word| expanded(&word.to_ascii_lowercase())).collect();

    assert!(!found.is_empty(), "a name holds at least one word");
    assert!(found.iter().all(|word| !word.is_empty()), "a word is never empty");

    found
}

#[cfg(test)]
mod tests {
    use super::{constant_name, field_name, module_name, type_name, words};

    #[test]
    fn a_constant_shares_the_field_reordering_and_keeps_a_keyword() {
        assert_eq!(constant_name("maxResults"), "RESULTS_MAX", "the bound goes last");
        assert_eq!(constant_name("type"), "TYPE", "a screaming keyword is no keyword");
        assert_eq!(constant_name("self"), "SELF", "a screaming keyword is no keyword");
        assert_eq!(constant_name("y"), "Y_VALUE", "a name too short to spell a word gains one");
        assert_eq!(constant_name("max-results"), "RESULTS_MAX", "a dash splits a word");

        assert_eq!(constant_name("ruleScopeARIs"), "RULE_SCOPE_ARIS", "a plural acronym");

        assert_eq!(
            constant_name("The title of the page."),
            "THE_TITLE_OF_THE_PAGE",
            "a sentence is a sentence of words",
        );
    }

    #[test]
    fn a_field_name_reads_big_endian() {
        assert_eq!(field_name("maxResults"), "results_max", "a bound goes last");
        assert_eq!(field_name("nextPageToken"), "page_token_next", "a bound goes last");
        assert_eq!(field_name("firstName"), "name_first", "a bound goes last");
        assert_eq!(field_name("totalPageCount"), "page_count_total", "measure then bound");
        assert_eq!(field_name("maxCount"), "count_max", "a measure outranks a bound");
        assert_eq!(field_name("sizeBytesMax"), "size_bytes_max", "already in order");
        assert_eq!(field_name("body-format"), "body_format", "no qualifier, no change");
        assert_eq!(field_name("spaceKey"), "space_key", "no qualifier, no change");
    }

    #[test]
    fn a_field_name_spells_its_words_out_and_avoids_keywords() {
        assert_eq!(field_name("appInfo"), "app_information", "an abbreviation expands");
        assert_eq!(field_name("params"), "parameters", "an abbreviation expands");
        assert_eq!(field_name("type"), "kind", "a reserved word has one replacement");
        assert_eq!(field_name("match"), "match_value", "any other keyword gains a noun");
        assert_eq!(field_name("x"), "x_value", "a name too short to spell a word gains one");
        assert_eq!(field_name("id"), "id", "an acronym is understood as it stands");
    }

    #[test]
    fn a_module_name_follows_the_type_it_holds() {
        assert_eq!(module_name("GetPageByID"), "get_page_by_id", "snake case");
        assert_eq!(module_name("SearchByCQL"), "search_by_cql", "a run");
        assert_eq!(module_name("CQLPersonalData"), "cql_personal_data", "a leading run");
    }

    #[test]
    fn a_type_name_capitalizes_each_acronym_as_one_word() {
        assert_eq!(type_name("getPageById"), "GetPageByID", "the first word rises");
        assert_eq!(type_name("searchContentByCql"), "SearchContentByCQL", "a word");
        assert_eq!(type_name("LinkUrlsBean"), "LinkURLsBean", "a plural acronym");
        assert_eq!(type_name("ContentIDs"), "ContentIDs", "a plural acronym survives");
        assert_eq!(type_name("ruleScopeARIs"), "RuleScopeARIs", "another plural acronym");
        assert_eq!(type_name("CQL - Search content"), "CQLSearchContent", "a sentence collapses");
        assert_eq!(type_name("UIModificationDetails"), "UIModificationDetails", "a leading run");
    }

    #[test]
    fn a_word_ends_at_every_change_a_name_can_spell() {
        assert_eq!(words("spaceIdOrKey"), ["space", "Id", "Or", "Key"], "camel humps");
        assert_eq!(words("CQLPersonalData"), ["CQL", "Personal", "Data"], "a leading run");
        assert_eq!(words("max-results"), ["max", "results"], "a dash");
        assert_eq!(words("a_b c"), ["a", "b", "c"], "an underscore and a space");
        assert_eq!(words("Oauth2Token"), ["Oauth2", "Token"], "a digit then a hump");
    }
}
