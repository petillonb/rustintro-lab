use std::ops::{Add, Div, Mul, Sub};

use crate::error::{Expr, Operator};

impl Add for Expr {
  type Output = Expr;

  fn add(self, rhs: Self) -> Self::Output {
    todo!()
  }
}

impl Mul for Expr {
  type Output = Expr;

  fn mul(self, rhs: Self) -> Self::Output {
    todo!()
  }
}

impl Div for Expr {
  type Output = Expr;

  fn div(self, rhs: Self) -> Self::Output {
    todo!()
  }
}

impl Sub for Expr {
  type Output = Expr;

  fn sub(self, rhs: Self) -> Self::Output {
    todo!()
  }
}

struct VarIterator {
  // TODO: fill this
}

impl From<Expr> for VarIterator {
  fn from(value: Expr) -> Self {
    todo!()
  }
}

impl Iterator for VarIterator {
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    todo!()
  }
}

#[cfg(test)]
mod test {
  use std::collections::HashMap;

  use super::*;
  use crate::error::eval;

  #[test]
  fn expr_traits() {
    let e: Expr = (((Expr::from(5) + 3.into()) * 2.into()) - 4.into()) / 2.into();
    let r = eval(&HashMap::default(), &e);
    assert_eq!(r, Ok(6));
  }

  #[test]
  fn var_iterator_a() {
    let e: Expr = Expr::from("x") + ((Expr::from("a") * "b".into()) / "c".into()) + "d".into();
    let v = VarIterator::from(e).collect::<Vec<String>>().join(" ");
    assert_eq!(v, "x a b c d");
  }

  #[test]
  fn var_iterator_b() {
    let e: Expr = Expr::from(3) + (Expr::from("a") - Expr::from("b") / "c".into()) + "d".into();
    let v = VarIterator::from(e).collect::<Vec<String>>().join(" ");
    assert_eq!(v, "a b c d");
  }
}
