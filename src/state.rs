use std::cell::RefCell;
use std::collections::HashMap;
use types::{Value, Error};
use parser::{Rule, Tree};

pub struct State {
    pub vars: HashMap<String, Tree>,
    cache: RefCell<HashMap<String, Value>>
}

impl State {
    pub fn new() -> State {
        State {
            vars: HashMap::new(),
            cache: RefCell::new(HashMap::new())
        }
    }

    pub fn set_var(&mut self, varname: String, expr: Tree) {
        self.cache.replace(HashMap::new());
        self.vars.insert(varname, expr);
    }

    pub fn cache_var(&self, varname: String, value: Value) {
        self.cache.borrow_mut().insert(varname, value);
    }

    pub fn resolve_expression(&self, expr: &Tree) -> Result<Value, Error> {
        match expr.rule {
            Rule::sum_expr => {
                (0..).map(|i| 1 + i * 2)
                .take_while(|&i| i < expr.children.len())
                .try_fold(
                    self.resolve_expression(&expr.children[0])?,
                    |acc, i| {
                        let resolved = self.resolve_expression(&expr.children[i + 1])?;

                        match expr.children[i].text.as_str() {
                            "+" => acc.add(&resolved),
                            "-" => acc.sub(&resolved),
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
                            "/" => acc.div(&resolved),
                            _ => acc.mul(&resolved)
                        }
                    }
                )
            },
            Rule::exp_expr => {
                self.resolve_expression(&expr.children[0])?
                .exp(&self.resolve_expression(&expr.children[2])?)
            },
            Rule::negate_expr => self.resolve_expression(&expr.children[0])?.negate(),
            Rule::identifier => {
                if let Some(value) = self.cache.borrow().get(&expr.text) {
                    return Ok(value.clone());
                }

                let result = match self.vars.get(&expr.text) {
                    Some(expr) => self.resolve_expression(expr),
                    None => Err(Error::ResolveError)
                };

                self.cache.borrow_mut().insert(expr.text.clone(), result.clone()?);

                result
            },
            Rule::number_lit => match expr.text.parse::<f64>() {
                Ok(x) => Ok(Value::Number(x)),
                Err(_) => Err(Error::ResolveError)
            },
            Rule::matrix_lit => {
                let rows = &expr.children;
                let height = rows.len();

                if height == 0 {
                    return Ok(Value::Matrix(vec![], 0));
                }

                let width = rows[0].children.len();

                if rows.iter().any(|row| {
                    row.rule != Rule::matrix_row_lit
                    || row.children.len() != width
                }) {
                    return Err(Error::DefinitionError);
                }

                let data = rows.iter()
                    .flat_map(|row| row.children.iter())
                    .try_fold(vec![], |mut acc, x| {
                        acc.push(self.resolve_expression(x)?);
                        Ok(acc)
                    })?;

                Ok(Value::Matrix(data, width))
            },
            _ => Err(Error::ResolveError)
        }
    }
}
