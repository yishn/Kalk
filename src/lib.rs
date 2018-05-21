extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;
pub mod types;
mod state;

pub use state::State;

#[test]
fn parse_simple_statement() {
    let mut state = State::new();
    let trees = parser::parse_rule(parser::Rule::expression, "[0, 1; 1, x; 1, 0] * [1, 0, 0; 0, 1, 0]").unwrap();

    state.set_var("x".to_string(), parser::Tree {
        rule: parser::Rule::number_lit,
        text: "2".to_string(),
        children: vec![]
    });

    println!("{:#?}", trees[0]);
    println!("{:#?}", state.resolve_expression(&trees[0]));
}
