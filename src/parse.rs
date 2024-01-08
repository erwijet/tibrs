use std::fs;

use anyhow::{Context, Result};
use itertools::Itertools;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{TibToken, TokenMap};

#[derive(Parser)]
#[grammar = "../grammar.peg"]
pub struct TibParser;

static RAW_TOKS: &'static str = include_str!("../toks.json");

pub fn pair_to_str<'lt>(p: &Pair<'lt, Rule>) -> &'lt str {
    match p.as_str() {
        "\n" => "@@NEWLINE",
        s => s,
    }
}

pub fn parse_str(tib: &str) -> Result<Vec<TibToken>> {
    let token_map: TokenMap = serde_json::from_str(RAW_TOKS).unwrap();
    let pairs = TibParser::parse(Rule::program, tib)?.nth(0).unwrap();

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

    Ok(tokens?)
}
