use crate::types::{Expr, Op, Environment};

fn env_get(env: &Environment, target: &String) -> Option<Expr> {
  env.iter().find(|(key, _)| key == target).map(|(_, expr)| expr.clone())
}

fn env_insert(env: &mut Environment, name: &String, value: &Expr) {
  // Search for existing name
  for (key, val) in env.iter_mut() {
    if key == name {
      *val = value.clone();
      return;
    }
  }

  env.push((name.clone(), value.clone()));
}

pub fn eval_expr(expr: &Expr, env: &mut Environment) -> Result<Expr, String> {
  match expr {
    // Int
    Expr::Int(n) => Ok(Expr::Int(*n)),

    // Float
    Expr::Float(d) => Ok(Expr::Float(*d)),

    // String
    Expr::String(s) => Ok(Expr::String(s.clone())),

    // Bool
    Expr::Bool(b) => Ok(Expr::Bool(*b)),

    // Var
    Expr::Var(v) => {
      match env_get(env, v) {
        Some(e) => eval_expr(&e, env),
        None => Err(format!("NameError: name {} is not defined", v))
      }
    }
    
    // VarAssign
    // *** In the future, might want to change return type of interpreter to something different than Expr
    // *** Since there are statements and expressions in Python. Variable assignments like this technically 
    // *** Don't return anything in Python
    Expr::VarAssign(v, e ) => {
      match eval_expr(e, env) {
        Ok(eval) => {
          // replace env with new value is name already exists, otherwise push new entry to end
          env_insert(env, v, &eval);
          Ok(eval)
        },
        Err(e) => Err(e)
      }
    }

    // Binop
    Expr::Binop(op, left, right) => {
      match (eval_expr(left, env), eval_expr(right, env)) {
        (Ok(left_eval), Ok(right_eval)) => {
          eval_binop(op, &left_eval, &right_eval)
        },
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
      }
    },
    _ => Err("SyntaxError: unexpected expression".to_string())
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

    // Less than
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