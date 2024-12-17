use crate::types::{Expr, Op};

pub fn eval_expr(expr: &Expr) -> Result<Expr, String> {
  match expr {
    // Int
    Expr::Int(n) => Ok(Expr::Int(*n)),
    // Float
    Expr::Float(d) => Ok(Expr::Float(*d)),
    // String
    Expr::String(s) => Ok(Expr::String(s.clone())),
    // Bool
    Expr::Bool(b) => Ok(Expr::Bool(*b)),
    // Binop
    Expr::Binop(op, left, right) => {
      match (eval_expr(left), eval_expr(right)) {
        (Ok(left_eval), Ok(right_eval)) => {
          eval_binop(op, &left_eval, &right_eval)
        },
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
      }
    }
  }
}

fn eval_binop(op: &Op, left: &Expr, right: &Expr) -> Result<Expr, String> {
  match &op {
    // Addition
    Op::Add => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Int(n1 + n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Float((*n1 as f32) + n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Float(n1 + (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Float(n1 + n2)),
        (Expr::String(s1), Expr::String(s2)) => {Ok(Expr::String(s1.clone() + s2))}, // String concatenation
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} + {}", left, right))
      }
    },

    // Subtraction
    Op::Sub => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Int(n1 - n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Float((*n1 as f32) - n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Float(n1 - (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Float(n1 - n2)),
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} - {}", left, right))
      }
    },

    // Multiplication
    Op::Mult => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Int(n1 * n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Float((*n1 as f32) * n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Float(n1 * (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Float(n1 * n2)),
        // String multiplication
        (Expr::String(s), Expr::Int(n)) => {
          let mut concat = String::new();
          for _ in 0..*n {
            concat.push_str(s);
          }
          Ok(Expr::String(concat))
        },

        _ => Err(format!("TypeError: Invalid type(s) evaluating {} * {}", left, right))
      }
    },
    // Division
    Op::Div => {
      match (&left, &right) {
        (_, Expr::Int(0)) | (_, Expr::Float(0.0)) => Err(format!("ZeroDivisionError: division by zero")),
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Float((*n1 as f32) / (*n2 as f32))),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Float((*n1 as f32) / n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Float(n1 / (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Float(n1 / n2)),
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} / {}", left, right))
      }
    },
    // Or
    Op::Or => {
      match left.to_bool() {
        Ok(b) => if !b {Ok(right.clone())} else {Ok(left.clone())},
        Err(e) => Err(e),
      }
    },
    // And
    Op::And => {
      match left.to_bool() {
        Ok(b) => if !b {Ok(left.clone())} else {Ok(right.clone())},
        Err(e) => Err(e),
      }
    },
    // Equals
    Op::Equal => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Bool(n1 == n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Bool((*n1 as f32) == *n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Bool(*n1 == (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Bool(n1 == n2)),
        (Expr::Bool(b1), Expr::Bool(b2)) => Ok(Expr::Bool(b1 == b2)),
        (Expr::String(s1), Expr::String(s2)) => Ok(Expr::Bool(s1 == s2)),
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} == {}", left, right))
      }
    }
    // Not Equals
    Op::NotEqual => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Bool(n1 != n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Bool((*n1 as f32) != *n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Bool(*n1 != (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Bool(n1 != n2)),
        (Expr::Bool(b1), Expr::Bool(b2)) => Ok(Expr::Bool(b1 != b2)),
        (Expr::String(s1), Expr::String(s2)) => Ok(Expr::Bool(s1 == s2)),
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} != {}", left, right))
      }
    }

    // Less than (could add string comparisons)
    Op::Less => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Bool(n1 < n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Bool((*n1 as f32) < *n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Bool(*n1 < (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Bool(n1 < n2)),
        (Expr::Bool(b1), Expr::Bool(b2)) => Ok(Expr::Bool(b1 < b2)),
        (Expr::String(s1), Expr::String(s2)) => Ok(Expr::Bool(s1 < s2)),
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} < {}", left, right))
      }
    }
    // Greater than
    Op::Greater => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Bool(n1 > n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Bool((*n1 as f32) > *n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Bool(*n1 > (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Bool(n1 > n2)),
        (Expr::Bool(b1), Expr::Bool(b2)) => Ok(Expr::Bool(b1 > b2)),
        (Expr::String(s1), Expr::String(s2)) => Ok(Expr::Bool(s1 > s2)),
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} > {}", left, right))
      }
    }
    // Less than or equal
    Op::LessEqual => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Bool(n1 <= n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Bool((*n1 as f32) <= *n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Bool(*n1 <= (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Bool(n1 <= n2)),
        (Expr::Bool(b1), Expr::Bool(b2)) => Ok(Expr::Bool(b1 <= b2)),
        (Expr::String(s1), Expr::String(s2)) => Ok(Expr::Bool(s1 <= s2)),
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} <= {}", left, right))
      }
    }
    // Greater than or equal
    Op::GreaterEqual => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Bool(n1 >= n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Bool((*n1 as f32) >= *n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Bool(*n1 >= (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Bool(n1 >= n2)),
        (Expr::Bool(b1), Expr::Bool(b2)) => Ok(Expr::Bool(b1 >= b2)),
        (Expr::String(s1), Expr::String(s2)) => Ok(Expr::Bool(s1 >= s2)),
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} >= {}", left, right))
      }
    }
  }
}