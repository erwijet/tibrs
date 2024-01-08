use itertools::Itertools;
use std::{fs, path::PathBuf, str::FromStr};

include!("./src/models.rs");

fn build(in_tokmap: &'static str, out_grammar: &'static str) {
    let map: TokenMap = serde_json::from_str(&*fs::read_to_string(in_tokmap).unwrap()).unwrap();
    let grammar_path = PathBuf::from_str(out_grammar).unwrap();

    if grammar_path.exists() {
        fs::remove_file(&grammar_path).unwrap();
    }

    let keys = map
        .keys()
        .sorted_by(|a, b| {
            Ord::cmp(
                &map.get(*b).unwrap().text.len(),
                &map.get(*a).unwrap().text.len(),
            )
        })
        .into_iter()
        .join(" | ");

    let rules = map
        .iter()
        .map(|(k, v)| {
            format!(
                "{k} = @{{ \"{}\" }}",
                v.text.replace("\"", "\\\"").replace("@@NEWLINE", "\\n")
            )
        })
        .join("\n");

    let contents = format!("// GENERATED FILE\n\nWHITESPACE = {{ \"\\t\" }}\nprogram = {{ SOI ~ token* ~ EOI }}\ntoken = _{{ {keys} }}\n\n// -- //\n\n{rules}");

    fs::write(grammar_path, contents).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=toks.json");
    build("./toks.json", "./grammar.peg");
}
