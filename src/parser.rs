use pest::{Parser, Error as PestError};
use pest::iterators::Pairs;

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("../data/kalk_grammar.pest");

#[derive(Parser)]
#[grammar = "../data/kalk_grammar.pest"]
struct KalkParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, PestError<Rule>> {
    KalkParser::parse(Rule::document, input)
}
