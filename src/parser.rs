use pest::{Parser, Error as PestError};
use pest::iterators::Pairs;

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("../data/kalk_grammar.pest");

#[derive(Parser)]
#[grammar = "../data/kalk_grammar.pest"]
struct KalkParser;

pub type PestResult<'a, T> = Result<T, PestError<'a, Rule>>;

pub fn parse(input: &str) -> PestResult<Pairs<Rule>> {
    parse_rule(Rule::document, input)
}

pub fn parse_rule(rule: Rule, input: &str) -> PestResult<Pairs<Rule>> {
    KalkParser::parse(rule, input)
}
