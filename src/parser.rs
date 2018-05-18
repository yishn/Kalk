use pest::{self, Parser, Error as PestError};

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("../data/kalk_grammar.pest");

pub type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;
pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;
pub type PestResult<'a, T> = Result<T, PestError<'a, Rule>>;

#[derive(Parser)]
#[grammar = "../data/kalk_grammar.pest"]
struct KalkParser;

pub fn parse(input: &str) -> PestResult<Pairs> {
    parse_rule(Rule::document, input)
}

pub fn parse_rule(rule: Rule, input: &str) -> PestResult<Pairs> {
    KalkParser::parse(rule, input)
}
