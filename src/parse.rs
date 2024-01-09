use anyhow::{Context, Result};
use itertools::Itertools;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{TibToken, TokenMap};

#[derive(Parser)]
#[grammar = "../grammar.peg"]
pub struct TibParser;

static RAW_TOKS: &str = include_str!("../toks.json");

pub fn pair_to_str<'lt>(p: &Pair<'lt, Rule>) -> &'lt str {
    match p.as_str() {
        "\n" => "@@NEWLINE",
        s => s,
    }
}

pub fn get_tok_map() -> TokenMap {
    serde_json::from_str(RAW_TOKS).unwrap()
}

pub fn parse_str(tib: &str) -> Result<Vec<TibToken>> {
    let preprocessed_source = tib.split('\n').map(|line| line.trim_start()).join("\n");
    let token_map = get_tok_map();
    let pairs = TibParser::parse(Rule::program, &preprocessed_source)?.next()
        .unwrap();

    let tokens: Result<Vec<TibToken>> = pairs
        .into_inner()
        .filter_map(|pair| {
            if let Rule::WHITESPACE | Rule::EOI = pair.as_rule() {
                return None; // skip
            }

            Some(
                token_map
                    .get(&*format!("{:?}", pair.as_rule()))
                    .with_context(|| format!("Failed to look up token: '{:?}'", pair.as_rule()))
                    .map(|borrowed| borrowed.clone()),
            )
        })
        .collect();

    tokens
}
