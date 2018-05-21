use std::fmt;
use self::Value::*;

#[derive(Debug)]
pub enum Error {
    ArithmeticError,
    DefinitionError,
    ResolveError
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Matrix(Vec<Value>, usize)
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Number(a) => write!(f, "{}", a),
            &Matrix(ref data, _) => {
                let (height, width) = self.size();
                let mut result = String::new();

                result.push('[');

                for i in 0..height {
                    if i > 0 {
                        result.push_str(match f.alternate() {
                            true => ";\n ",
                            false => "; "
                        });
                    }

                    for j in 0..width {
                        if j > 0 {
                            result.push_str(", ");
                        }

                        result.push_str(&format!("{:?}", data[i * width + j]))
                    }
                }

                result.push(']');

                write!(f, "{}", result)
            }
        }
    }
}

impl Value {
    pub fn size(&self) -> (usize, usize) {
        match self {
            &Number(_) => (1, 1),
            &Matrix(ref data, width) => match (width, data.len()) {
                (0, _) => (0, 0),
                (_, 0) => (0, 0),
                (width, len) => (len / width, width)
            }
        }
    }

    pub fn negate(&self) -> Result<Value, Error> {
        match self {
            &Number(a) => Ok(Number(-a)),
            &Matrix(ref data, width) => Ok(Matrix(
                data.iter()
                .try_fold(vec![], |mut acc, x| {
                    acc.push(x.negate()?);
                    Ok(acc)
                })?,
                width
            ))
        }
    }

    pub fn add(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (&Number(a), &Number(b)) => Ok(Number(a + b)),
            (&Matrix(ref a, width), &Matrix(ref b, _)) if self.size() == other.size() => {
                let data = a.iter()
                    .enumerate()
                    .try_fold(vec![], |mut acc, (i, x)| {
                        acc.push(x.add(&b[i])?);
                        Ok(acc)
                    })?;

                Ok(Matrix(data, width))
            },
            _ => Err(Error::ArithmeticError)
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, Error> {
        self.add(&other.negate()?)
    }

    pub fn mul(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (&Number(a), &Number(b)) => Ok(Number(a * b)),
            (&Number(_), &Matrix(ref b, width)) => {
                let data = b.iter().try_fold(vec![], |mut acc, x| {
                    acc.push(self.mul(x)?);
                    Ok(acc)
                })?;

                Ok(Matrix(data, width))
            },
            (&Matrix(_, _), &Number(_)) => other.mul(self),
            (&Matrix(_, inner), &Matrix(_, _)) if inner == 0 => Ok(Matrix(vec![], 0)),
            (&Matrix(ref a, inner), &Matrix(ref b, width)) if inner == other.size().0 => {
                let height = self.size().0;
                let data = (0..width * height).try_fold(vec![], |mut acc, index| {
                    let (i, j) = (index / width, index % width);

                    acc.push(
                        (0..inner)
                        .try_fold(None, |acc: Option<Value>, k| {
                            let value = a[i * inner + k].mul(&b[k * width + j])?;
                            match acc {
                                Some(x) => Ok(Some(x.add(&value)?)),
                                None => Ok(Some(value))
                            }
                        })?
                        .unwrap()
                    );

                    Ok(acc)
                })?;

                Ok(Matrix(data, width))
            },
            _ => Err(Error::ArithmeticError)
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (&Number(a), &Number(b)) if b != 0.0 => Ok(Number(a / b)),
            _ => Err(Error::ArithmeticError)
        }
    }

    pub fn exp(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (&Number(a), &Number(b)) => Ok(Number(a.powf(b))),
            _ => Err(Error::ArithmeticError)
        }
    }
}
