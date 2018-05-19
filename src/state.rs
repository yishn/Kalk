use std::collections::HashMap;
use types::Error;
use parser::{Rule, Tree};

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

    pub fn resolve_expression(&self, expr: &Tree) -> Result<f64, Error> {
        match expr.rule {
            Rule::sum_expr => {
                (0..).map(|i| 1 + i * 2)
                .take_while(|&i| i < expr.children.len())
                .try_fold(
                    self.resolve_expression(&expr.children[0])?,
                    |acc, i| {
                        let resolved = self.resolve_expression(&expr.children[i + 1])?;

                        match expr.children[i].text.as_str() {
                            "+" => Ok(acc + resolved),
                            "-" => Ok(acc - resolved),
                            _ => Err(Error::ResolveError)
                        }
                    }
                )
            },
            Rule::prod_expr => {
                (1..)
                .take_while(|&i| i < expr.children.len())
                .filter(|&i| expr.children[i].rule != Rule::operator)
                .try_fold(
                    self.resolve_expression(&expr.children[0])?,
                    |acc, i| {
                        let resolved = self.resolve_expression(&expr.children[i])?;

                        match expr.children[i - 1].text.as_str() {
                            "/" => Ok(acc / resolved),
                            _ => Ok(acc * resolved)
                        }
                    }
                )
            },
            Rule::exp_expr => Ok(
                self.resolve_expression(&expr.children[0])?
                .powf(self.resolve_expression(&expr.children[2])?)
            ),
            Rule::negate_expr => Ok(
                -self.resolve_expression(&expr.children[0])?
            ),
            Rule::identifier => match self.vars.get(&expr.text) {
                Some(expr) => self.resolve_expression(expr),
                None => Err(Error::ResolveError)
            },
            Rule::number_lit => match expr.text.parse::<f64>() {
                Ok(x) => Ok(x),
                Err(_) => Err(Error::ResolveError)
            },
            _ => Err(Error::ResolveError)
        }
    }
}
