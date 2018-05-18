use std::collections::HashMap;
use types::Expression;

pub struct State<'a> {
    variables: HashMap<&'a str, Expression<'a>>
}

impl<'a> State<'a> {
    pub fn new() -> State<'a> {
        State {variables: HashMap::new()}
    }
}
