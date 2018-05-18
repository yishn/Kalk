use std::fmt;
use pest::{Parser, Error as PestError};
use pest::iterators::Pair;

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("../data/kalk_grammar.pest");

#[derive(Parser)]
#[grammar = "../data/kalk_grammar.pest"]
struct KalkParser;

pub type ParseResult<'a> = Result<Vec<Tree>, PestError<'a, Rule>>;

#[derive(Debug, Clone)]
pub enum Content {
    Text(String),
    Children(Vec<Tree>)
}

#[derive(Clone)]
pub struct Tree {
    rule: Rule,
    content: Content
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{}", self.rule, match self.content {
            Content::Text(ref x) => format!("({:#?})", x),
            Content::Children(ref x) => format!(" {:#?}", x)
        })
    }
}

pub trait ToTree {
    fn to_syntax_tree(self) -> Tree;
}

impl<'a> ToTree for Pair<'a, Rule> {
    fn to_syntax_tree(self) -> Tree {
        let rule = self.as_rule();
        let text = self.as_str();
        let children = self.into_inner()
            .map(|pair| pair.to_syntax_tree())
            .collect::<Vec<_>>();

        Tree {
            rule,
            content: match children.len() {
                0 => Content::Text(text.to_string()),
                _ => Content::Children(children)
            }
        }
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
