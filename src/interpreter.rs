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

    // Function Call
    PyType::Expr(Expr::FunctionCall(func_name, arguments)) => eval_function_call(func_name, arguments, env),

    // Return
    PyType::Expr(Expr::Return(e)) => {
      match evaluate(e, env) {
        Ok(eval) => {
          if let PyType::Expr(_) = eval {
            return Ok(PyType::Expr(Expr::Return(Box::from(eval))));
          }
          return Err("TypeError: Return value is not an expression".to_string());
        },
        Err(e) => Err(e)
      }
    }

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

    // Function Definition
    PyType::Stmt(Stmt::Function(func_name, _, _)) => {
      env_insert(env, func_name, expr);
      Ok(expr.clone())
    },

    _ => Err("SyntaxError: unexpected expression".to_string())

    
  }
}

fn eval_function_call(func_name: &String, arguments: &Vec<PyType>, env: &mut Environment) -> Result<PyType, String> {
  // Get function from environment, throw error if not there
  // Check if same # of params & args
  // For each parameter, add var & matching argument to environment 
  // Evaluate function body line by line, return Expr if return Expr found

  match env_get(env, func_name) {
    Some(PyType::Stmt(Stmt::Function(_, parameters, body))) => {
      if arguments.len() != parameters.len() {
        return Err(format!("TypeError: {} takes {} positional arguments but {} were given", func_name, parameters.len(), arguments.len()))
      }

      // Add arguments & parameters to environment
      for i in 0..parameters.len() {
        let eval_result = evaluate(&arguments[i], env);

        if let Err(e) = eval_result {
          return Err(e);
        }
        if let Ok(PyType::Expr(eval_arg)) = eval_result {
          env_insert(env, &parameters[i], &PyType::Expr(eval_arg))
        } else {
          return Err("TypeError: argument does not evaluate to an expression".to_string());
        }
      }

      // Evaluate function line-by-line, return Expr if return found
      // Value returned only if Return Expr found
      for i in 0..body.len() {
        let eval_line = evaluate(&body[i], env);
        if let Err(e) = eval_line {
          return Err(e);
        }
        // Return expression found, return function here
        if let Ok(PyType::Expr(Expr::Return(expr))) = eval_line {
          return Ok(*expr);
        }
      }
      return Ok(PyType::Stmt(Stmt::None));
    },
    Some(_) => Err("TypeError: object is not callable".to_string()),
    None => Err(format!("NameError: name {} is not defined", func_name))
  }
}

fn eval_if(condition: &Box<PyType>, body: &Vec<PyType>, else_body: &Option<Vec<PyType>>, env: &mut Environment) -> Result<PyType, String> {
  // Check if condition evaluates to boolean
  match evaluate(condition, env) { 
    Ok(PyType::Expr(Expr::Bool(b))) => {
      if b {
        // Process all lines, return result of last line
        for i in 0..(body.len() - 1) {
          let eval_line = evaluate(&body[i], env);

          if let Err(e) = eval_line { 
            return Err(e);
          }
          // Return found
          else if let Ok(PyType::Expr(Expr::Return(_))) = eval_line {
            return eval_line;
          }
        }
        let result = evaluate(&body[body.len() - 1], env);
        return result;
      } else {
        // Interpret else statement if it exists
        if let Some(else_body_list) = else_body {
          for i in 0..(else_body_list.len() - 1) {
            let line_eval = evaluate(&else_body_list[i], env);
            if let Err(e) = line_eval {
              return Err(e);
            }
            // Return found
            else if let Ok(PyType::Expr(Expr::Return(_))) = line_eval {
              return line_eval;
            }
          }
          let result = evaluate(&else_body_list[else_body_list.len() - 1], env);
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