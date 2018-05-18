extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;
pub mod types;
mod state;

pub use state::State;

#[test]
fn parse_simple_statement() {
    for pair in parser::parse_rule(parser::Rule::document, "y := -e^-x^2").unwrap() {
        println!("{}", pair);
    }
}
