extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;
pub mod types;
mod state;

pub use state::State;

#[test]
fn parse_simple_statement() {
    println!("{:#?}", parser::parse_rule(parser::Rule::document, "y := -x^5 + (2x)^3 + 5").unwrap());
}
