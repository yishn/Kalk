extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;

#[test]
fn parse_simple_statement() {
    for pair in parser::parse("y := 2 x y").unwrap() {
        println!("{}", pair);
    }
}
