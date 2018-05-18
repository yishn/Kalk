use std::collections::HashMap;
use parser::{Tree, Content};
use types::Error;

pub struct State {
    vars: HashMap<String, Tree>
}

impl State {
    pub fn new() -> State {
        State {vars: HashMap::new()}
    }

    pub fn set_var(&mut self, varname: String, expr: Tree) {
        self.vars.insert(varname, expr);
    }

    pub fn resolve_var(&self, varname: &str) -> Result<Tree, Error> {
        Err(Error::ResolveError)
    }
}
