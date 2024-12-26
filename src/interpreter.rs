use crate::types::{PyType, Stmt, Expr, Op, Environment};

fn env_get(env: &Environment, target: &String) -> Option<PyType> {
  env.iter().find(|(key, _)| key == target).map(|(_, expr)| expr.clone())
}

fn env_insert(env: &mut Environment, name: &String, value: &PyType) {
  // Search for existing name
  for (key, val) in env.iter_mut() {
    if key == name {
      *val = value.clone();
      return;
    }
  }

  env.push((name.clone(), value.clone()));
}

// Update original environment with new variable values (do not add new variable names)
// fn env_update(env: &mut Environment, closure: &mut Environment) {
//   // Update existing variables
//   for (target, old_val) in env.iter_mut() {
//     for (key, new_val) in closure.iter() {
//       if target == key && *old_val != *new_val {
//         *old_val = new_val.clone();
//         return;
//       }
//     }
//   }
// }

pub fn evaluate(expr: &PyType, env: &mut Environment) -> Result<PyType, String> {
  match expr {

    // *** EXPRESSIONS ***

    // Int
    PyType::Expr(Expr::Int(n)) => Ok(PyType::Expr(Expr::Int(*n))),

    // Float
    PyType::Expr(Expr::Float(d)) => Ok(PyType::Expr(Expr::Float(*d))),

    // String
    PyType::Expr(Expr::String(s)) => Ok(PyType::Expr(Expr::String(s.clone()))),

    // Bool
    PyType::Expr(Expr::Bool(b)) => Ok(PyType::Expr(Expr::Bool(*b))),

    // Var
    PyType::Expr(Expr::Var(v)) => {
      match env_get(env, v) {
        Some(e) => evaluate(&e, env),
        None => Err(format!("NameError: name {} is not defined", v))
      }
    }

    // Not
    PyType::Expr(Expr::Not(e)) => {
      match evaluate(e, env) {
        Ok(eval) => {
          match eval.to_bool() {
            Ok(b) => Ok(PyType::Expr(Expr::Bool(!b))),
            Err(e) => Err(e)
          }
        },
        Err(e) => Err(e)
      }
    }
    
    // Binop
    PyType::Expr(Expr::Binop(op, left, right)) => {
      match (evaluate(left, env), evaluate(right, env)) {
        (Ok(left_eval), Ok(right_eval)) => {
          eval_binop(op, &left_eval, &right_eval)
        },
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
      }
    },

    // *** STATEMENTS ***

    // VarAssign
    PyType::Stmt(Stmt::VarAssign(v, e )) => {
      match evaluate(e, env) {
        Ok(PyType::Expr(eval)) => {
          // replace env with new value is name already exists, otherwise push new entry to end
          env_insert(env, v, &PyType::Expr(eval.clone()));
          Ok(PyType::Stmt(Stmt::VarAssign(v.to_string(), Box::from(PyType::Expr(eval)))))
        },
        Ok(_) => Err("TypeError: variable not assigned to expressions".to_string()),
        Err(e) => Err(e)
      }
    }

    // If-Else Statement
    PyType::Stmt(Stmt::If(condition, body, else_body)) => eval_if(condition, body, else_body, env),
    _ => Err("SyntaxError: unexpected expression".to_string())
  }
}

fn eval_if(condition: &Box<PyType>, body: &Vec<PyType>, else_body: &Option<Vec<PyType>>, env: &mut Environment) -> Result<PyType, String> {
  // let mut closure = env.clone();

  // Check if condition evaluates to boolean
  match evaluate(condition, env) { // closure
    Ok(PyType::Expr(Expr::Bool(b))) => {
      if b {
        // Process all lines, return result of last line
        for i in 0..(body.len() - 1) {
          if let Err(e) = evaluate(&body[i], env) { // closure
            return Err(e);
          }
        }
        let result = evaluate(&body[body.len() - 1], env); //closure
        // env_update(env, &mut closure);
        return result;
      } else {
        // Interpret else statement if it exists
        if let Some(else_body_list) = else_body {
          for i in 0..(else_body_list.len() - 1) {
            if let Err(e) = evaluate(&else_body_list[i], env) { // closure
              return Err(e);
            }
          }
          let result = evaluate(&else_body_list[else_body_list.len() - 1], env); //closure
          // env_update(env, &mut closure);
          return result;
        } else {
          return Ok(PyType::Stmt(Stmt::None));
        }
      }
    },
    Ok(_) => Err("SyntaxError: condition of if statement does not evaluate to boolean".to_string()),
    Err(e) => Err(e)
  }
}

fn eval_binop(op: &Op, left: &PyType, right: &PyType) -> Result<PyType, String> {
  match (left, right) {
    (PyType::Expr(left_expr), PyType::Expr(right_expr)) => {
      match &op {
        // Addition
        Op::Add => {
          match (left_expr, right_expr) {
            (Expr::Int(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Int(n1 + n2))),
            (Expr::Int(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Float((*n1 as f32) + n2))),
            (Expr::Float(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Float(n1 + (*n2 as f32)))),
            (Expr::Float(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Float(n1 + n2))),
            (Expr::String(s1), Expr::String(s2)) => Ok(PyType::Expr(Expr::String(s1.clone() + s2))), // String concatenation
            _ => Err(format!("TypeError: Invalid type(s) evaluating {} + {}", left_expr, right_expr))
          }
        },
    
        // Subtraction
        Op::Sub => {
          match (left_expr, right_expr) {
            (Expr::Int(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Int(n1 - n2))),
            (Expr::Int(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Float((*n1 as f32) - n2))),
            (Expr::Float(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Float(n1 - (*n2 as f32)))),
            (Expr::Float(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Float(n1 - n2))),
            _ => Err(format!("TypeError: Invalid type(s) evaluating {} - {}", left_expr, right_expr))
          }
        },
    
        // Multiplication
        Op::Mult => {
          match (left_expr, right_expr) {
            (Expr::Int(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Int(n1 * n2))),
            (Expr::Int(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Float((*n1 as f32) * n2))),
            (Expr::Float(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Float(n1 * (*n2 as f32)))),
            (Expr::Float(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Float(n1 * n2))),
            // String multiplication
            (Expr::String(s), Expr::Int(n)) => {
              let mut concat = String::new();
              for _ in 0..*n {
                concat.push_str(s);
              }
              Ok(PyType::Expr(Expr::String(concat)))
            },
    
            _ => Err(format!("TypeError: Invalid type(s) evaluating {} * {}", left_expr, right_expr))
          }
        },

        // Division
        Op::Div => {
          match (left_expr, right_expr) {
            (_, Expr::Int(0)) | (_, Expr::Float(0.0)) => Err(format!("ZeroDivisionError: division by zero")),
            (Expr::Int(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Float((*n1 as f32) / (*n2 as f32)))),
            (Expr::Int(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Float((*n1 as f32) / n2))),
            (Expr::Float(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Float(n1 / (*n2 as f32)))),
            (Expr::Float(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Float(n1 / n2))),
            _ => Err(format!("TypeError: Invalid type(s) evaluating {} / {}", left_expr, right_expr))
          }
        },

        // Or
        Op::Or => {
          match left.to_bool() { // left_expr
            Ok(b) => if !b {Ok(PyType::Expr(right_expr.clone()))} else {Ok(PyType::Expr(left_expr.clone()))},
            Err(e) => Err(e),
          }
        },

        // And
        Op::And => {
          match left.to_bool() { // left_expr
            Ok(b) => {
              if !b {
                Ok(PyType::Expr(Expr::Bool(false)))
              } else {
                Ok(PyType::Expr(right_expr.clone()))
              }
            },
            Err(e) => Err(e),
          }
        },

        // Equals
        Op::Equal => {
          match (left_expr, right_expr) {
            (Expr::Int(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(n1 == n2))),
            (Expr::Int(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool((*n1 as f32) == *n2))),
            (Expr::Float(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(*n1 == (*n2 as f32)))),
            (Expr::Float(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool(n1 == n2))),
            (Expr::Bool(b1), Expr::Bool(b2)) => Ok(PyType::Expr(Expr::Bool(b1 == b2))),
            (Expr::String(s1), Expr::String(s2)) => Ok(PyType::Expr(Expr::Bool(s1 == s2))),
            _ => Err(format!("TypeError: Invalid type(s) evaluating {} == {}", left_expr, right_expr))
          }
        },

        // Not Equals
        Op::NotEqual => {
          match (left_expr, right_expr) {
            (Expr::Int(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(n1 != n2))),
            (Expr::Int(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool((*n1 as f32) != *n2))),
            (Expr::Float(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(*n1 != (*n2 as f32)))),
            (Expr::Float(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool(n1 != n2))),
            (Expr::Bool(b1), Expr::Bool(b2)) => Ok(PyType::Expr(Expr::Bool(b1 != b2))),
            (Expr::String(s1), Expr::String(s2)) => Ok(PyType::Expr(Expr::Bool(s1 == s2))),
            _ => Err(format!("TypeError: Invalid type(s) evaluating {} != {}", left_expr, right_expr))
          }
        },
    
        // Less than
        Op::Less => {
          match (left_expr, right_expr) {
            (Expr::Int(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(n1 < n2))),
            (Expr::Int(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool((*n1 as f32) < *n2))),
            (Expr::Float(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(*n1 < (*n2 as f32)))),
            (Expr::Float(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool(n1 < n2))),
            (Expr::Bool(b1), Expr::Bool(b2)) => Ok(PyType::Expr(Expr::Bool(b1 < b2))),
            (Expr::String(s1), Expr::String(s2)) => Ok(PyType::Expr(Expr::Bool(s1 < s2))),
            _ => Err(format!("TypeError: Invalid type(s) evaluating {} < {}", left_expr, right_expr))
          }
        },

        // Greater than
        Op::Greater => {
          match (left_expr, right_expr) {
            (Expr::Int(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(n1 > n2))),
            (Expr::Int(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool((*n1 as f32) > *n2))),
            (Expr::Float(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(*n1 > (*n2 as f32)))),
            (Expr::Float(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool(n1 > n2))),
            (Expr::Bool(b1), Expr::Bool(b2)) => Ok(PyType::Expr(Expr::Bool(b1 > b2))),
            (Expr::String(s1), Expr::String(s2)) => Ok(PyType::Expr(Expr::Bool(s1 > s2))),
            _ => Err(format!("TypeError: Invalid type(s) evaluating {} > {}", left_expr, right_expr))
          }
        },

        // Less than or equal
        Op::LessEqual => {
          match (left_expr, right_expr) {
            (Expr::Int(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(n1 <= n2))),
            (Expr::Int(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool((*n1 as f32) <= *n2))),
            (Expr::Float(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(*n1 <= (*n2 as f32)))),
            (Expr::Float(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool(n1 <= n2))),
            (Expr::Bool(b1), Expr::Bool(b2)) => Ok(PyType::Expr(Expr::Bool(b1 <= b2))),
            (Expr::String(s1), Expr::String(s2)) => Ok(PyType::Expr(Expr::Bool(s1 <= s2))),
            _ => Err(format!("TypeError: Invalid type(s) evaluating {} <= {}", left_expr, right_expr))
          }
        },

        // Greater than or equal
        Op::GreaterEqual => {
          match (left_expr, right_expr) {
            (Expr::Int(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(n1 >= n2))),
            (Expr::Int(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool((*n1 as f32) >= *n2))),
            (Expr::Float(n1), Expr::Int(n2)) => Ok(PyType::Expr(Expr::Bool(*n1 >= (*n2 as f32)))),
            (Expr::Float(n1), Expr::Float(n2)) => Ok(PyType::Expr(Expr::Bool(n1 >= n2))),
            (Expr::Bool(b1), Expr::Bool(b2)) => Ok(PyType::Expr(Expr::Bool(b1 >= b2))),
            (Expr::String(s1), Expr::String(s2)) => Ok(PyType::Expr(Expr::Bool(s1 >= s2))),
            _ => Err(format!("TypeError: Invalid type(s) evaluating {} >= {}", left_expr, right_expr))
          }
        }
      }
    },
    _ => Err(format!("TypeError: Invalid type(s) evaluating {} {} {}", left, op, right))
  }
  
}