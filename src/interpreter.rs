use crate::types::{Expr, Op};

pub fn eval_expr(expr: &Expr) -> Result<Expr, String> {
  match expr {
    // Int
    Expr::Int(n) => Ok(Expr::Int(*n)),
    // Float
    Expr::Float(d) => Ok(Expr::Float(*d)),
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
  match (&left, &right) {
    (Expr::Int(n1), Expr::Int(n2)) => {
      match &op {
        Op::Add => Ok(Expr::Int(n1 + n2)),
        Op::Sub => Ok(Expr::Int(n1 - n2)),
        Op::Mult => Ok(Expr::Int(n1 * n2)),
        Op::Div => Ok(Expr::Int(n1 / n2)),
      }
    }
    // MATCH REST OF NUMERICAL TYPE COMBOS
    _ => Err(format!("Invalid type(s) evaluating {} {} {}", left, op, right))
  }
}
