use std::fmt;
use pest::{Parser, Error as PestError};
use pest::iterators::Pair;

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("../data/kalk_grammar.pest");

#[derive(Parser)]
#[grammar = "../data/kalk_grammar.pest"]
struct KalkParser;

pub type ParseResult<'a> = Result<Vec<Tree>, PestError<'a, Rule>>;

#[derive(Clone)]
pub struct Tree {
    pub rule: Rule,
    pub text: String,
    pub children: Vec<Tree>
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{}", self.rule, match self.children.len() {
            0 => format!("({:?})", self.text),
            _ => format!(" {:#?}", self.children)
        })
    }
}

pub trait ToTree {
    fn to_syntax_tree(self) -> Tree;
}

impl<'a> ToTree for Pair<'a, Rule> {
    fn to_syntax_tree(self) -> Tree {
        let rule = self.as_rule();
        let text = self.as_str().to_string();
        let children = self.into_inner()
            .map(|pair| pair.to_syntax_tree())
            .collect::<Vec<_>>();

        Tree {rule, text, children}
    }
}

pub fn parse(input: &str) -> ParseResult {
    parse_rule(Rule::document, input)
}

pub fn parse_rule(rule: Rule, input: &str) -> ParseResult {
    Ok(
        KalkParser::parse(rule, input)?
        .map(|pair| pair.to_syntax_tree())
        .collect()
    )
}
