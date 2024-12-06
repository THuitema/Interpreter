use crate::types::{Expr, Op};

pub fn eval_expr(expr: &Expr) -> Result<Expr, String> {
  match expr {
    // Int
    Expr::Int(n) => Ok(Expr::Int(*n)),
    // Float
    Expr::Float(d) => Ok(Expr::Float(*d)),
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
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} {} {}", left, op, right))
      }
    },
    // Subtraction
    Op::Sub => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Int(n1 - n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Float((*n1 as f32) - n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Float(n1 - (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Float(n1 - n2)),
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} {} {}", left, op, right))
      }
    },
    // Multiplication
    Op::Mult => {
      match (&left, &right) {
        (Expr::Int(n1), Expr::Int(n2)) => Ok(Expr::Int(n1 * n2)),
        (Expr::Int(n1), Expr::Float(n2)) => Ok(Expr::Float((*n1 as f32) * n2)),
        (Expr::Float(n1), Expr::Int(n2)) => Ok(Expr::Float(n1 * (*n2 as f32))),
        (Expr::Float(n1), Expr::Float(n2)) => Ok(Expr::Float(n1 * n2)),
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} {} {}", left, op, right))
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
        _ => Err(format!("TypeError: Invalid type(s) evaluating {} {} {}", left, op, right))
      }
    },
    // Or
    Op::Or => {
      match expr_to_bool(left) {
        Ok(Expr::Bool(b)) => if !b {Ok(right.clone())} else {Ok(left.clone())},
        Err(e) => Err(e),
        _ => Err(format!("TypeError: Invalid expression {} in \"or\" statement", left))
      }
    },
    // And
    Op::And => {
      match expr_to_bool(left) {
        Ok(Expr::Bool(b)) => if !b {Ok(left.clone())} else {Ok(right.clone())},
        Err(e) => Err(e),
        _ => Err(format!("TypeError: Invalid expression {} in \"and\" statement", left))
      }
    },
    _ => Err(format!("TypeError: Invalid type(s) evaluating {} {} {}", left, op, right))
  }
}

fn expr_to_bool(expr: &Expr) -> Result<Expr, String> {
  match expr {
    Expr::Bool(b) => Ok(Expr::Bool(*b)),
    Expr::Int(n) => if *n == 0 {Ok(Expr::Bool(false))} else {Ok(Expr::Bool(true))},
    Expr::Float(n) => if *n == 0.0 {Ok(Expr::Bool(false))} else {Ok(Expr::Bool(true))},
    _ => Err(format!("TypeError: Trying to convert {} to bool", {expr}))
  }
}

