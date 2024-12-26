use crate::lexer::tokenize; 
use crate::types::{Token, PyType, Stmt, Expr, Op};
use std::io::{self, Write};

fn lookahead(tokens: &Vec<Token>) -> Option<&Token> {
  tokens.get(0)
}

fn lookahead_at(tokens: &Vec<Token>, index: usize) -> Option<&Token> {
  tokens.get(index)
}

fn match_token(tokens: &Vec<Token>, token: &Token) -> Result<Vec<Token>, String> {
  match tokens.get(0) {
    Some(first_token) => {
      if first_token == token {
        Ok(tokens[1..].to_vec())
      } else {
        Err(format!("SyntaxError: Expected {}, but got {}", token, first_token))
      }
    }, 
    _ => Err(format!("SyntaxError: Expected {}, but reached end of tokens", token))
  }
}

fn read_body_line(prev_indent: &mut i32) -> Result<(Vec<Token>, i32), String> {
  print!("... ");
  io::stdout().flush().unwrap();
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  
  tokenize(&input, prev_indent)
}

pub fn parse(tokens: &Vec<Token>, prev_indent: &mut i32, indent_stack: &mut Vec<i32>) -> Result<(Vec<Token>, PyType), String> {
  match (lookahead(tokens), lookahead_at(tokens, 1)) {
    // AssignStatement
    (Some(Token::TokVar(v)), Some(Token::TokAssign)) => parse_assign(&match_token(&tokens, &Token::TokVar(v.clone())).unwrap(), v),
    
    // IfStatement
    (Some(Token::TokIf), _) => parse_if(&match_token(&tokens, &Token::TokIf).unwrap(), prev_indent, indent_stack),
    
    // Expr
    _ => parse_expr(tokens)
  }
}

fn parse_assign(tokens: &Vec<Token>, var_name: &String) -> Result<(Vec<Token>, PyType), String> {
  match parse_expr(&match_token(&tokens, &Token::TokAssign).unwrap()) {
    Ok((tokens2, e)) => Ok((tokens2, PyType::Stmt(Stmt::VarAssign(var_name.clone(), Box::from(e))))),
    Err(e) => Err(e)
  }
}

// Returns expression of if statement condition and list of expressions in body
fn parse_if(tokens: &Vec<Token>, prev_indent: &mut i32, indent_stack: &mut Vec<i32>) -> Result<(Vec<Token>, PyType), String> {
  indent_stack.push(*prev_indent);
  match parse_expr(tokens) {
    // Condition of if statement
    Ok((tokens2, condition)) => {
      match match_token(&tokens2, &Token::TokColon) {
        Ok(tokens3) if tokens3.is_empty() => {
          // Read first line of body under if statement and tokenize
          match read_body_line(prev_indent) {
            Ok((body_tokens, indentation)) => {
              *prev_indent = indentation;
              match lookahead(&body_tokens) {
                // Get indent for first line of body
                Some(Token::TokIndent(n)) => {

                  // Parsing body of if statement
                  let mut body_contents = Vec::<PyType>::new();
                  match parse_body(&match_token(&body_tokens, &Token::TokIndent(*n)).unwrap(), prev_indent, &mut body_contents, indent_stack) {
                    Ok((tokens4, _)) => {

                      // Check if there's an else statement to parse
                      if lookahead(&tokens4) == Some(&Token::TokElse) {
                        match parse_else(&match_token(&tokens4, &Token::TokElse).unwrap(), prev_indent, indent_stack) {
                          Ok((tokens5, else_body)) => return Ok((tokens5, PyType::Stmt(Stmt::If(Box::from(condition), body_contents, Some(else_body))))),
                          Err(e) => return Err(e)
                        }
                      }
                      else {
                        return Ok((tokens4, PyType::Stmt(Stmt::If(Box::from(condition), body_contents, None))))
                      }
                    },
                    Err(e) => Err(e)
                  }
                },
                _ => Err("Indentation Error: expected indent".to_string())
              }
            },
            Err(e) => Err(e)
          }
        },
        Err(e) => Err(e),
        _ => Err("SyntaxError: expected new line after ':'".to_string()) // if more tokens after :
      }
    },
    Err(e) => Err(e)
  }
}

fn parse_else(tokens: &Vec<Token>, prev_indent: &mut i32, indent_stack: &mut Vec<i32>) -> Result<(Vec<Token>, Vec<PyType>), String> {
  indent_stack.push(*prev_indent); // not sure if this is right
  
  match match_token(tokens, &Token::TokColon) {
    Ok(tokens2) if tokens2.is_empty() => {
      // Read first line of body and tokenize
      match read_body_line(prev_indent) {
        Ok((body_tokens, indentation)) => {
          *prev_indent = indentation;
          match lookahead(&body_tokens) {
            Some(Token::TokIndent(n)) => {
              // Parse rest of body
              let mut body_contents = Vec::<PyType>::new();
              match parse_body(&match_token(&body_tokens, &Token::TokIndent(*n)).unwrap(), prev_indent, &mut body_contents, indent_stack) {
                Ok((tokens3, _)) => {
                  Ok((tokens3, body_contents))
                },
                Err(e) => Err(e)
              }
            }
            _ => Err("Indentation Error: expected indent".to_string())
          }
        },
        Err(e) => Err(e)
      }
    },
    Err(e) => Err(e),
    _ => Err("SyntaxError: expected new line after ':'".to_string()) // if more tokens after :

  }
}


// Parses a closure (determined by indents in Python), returning a list of the expressions in it
fn parse_body(tokens: &Vec<Token>, prev_indent: &mut i32, body_contents: &mut Vec<PyType>, indent_stack: &mut Vec<i32>) -> Result<(Vec<Token>, Vec<PyType>), String> {
  match parse(tokens, prev_indent, indent_stack) {
    Ok((tokens2, parsed_line)) => {
      match lookahead(&tokens2) {

        Some(Token::TokDedent(n)) => {
          body_contents.push(parsed_line);
        
          let peek = indent_stack.last().unwrap();
          // Current line was unindented to return to the current scope
          if *n == *peek {
            indent_stack.pop();
            // Check if outer-most scope was exited
            if indent_stack.is_empty() {
              return Ok((match_token(&tokens2, &Token::TokDedent(*n)).unwrap(), body_contents.to_vec()));
            }
            return parse_body(&match_token(&tokens2, &Token::TokDedent(*n)).unwrap(), prev_indent, body_contents, indent_stack);
          } else if *n < *peek { 
            // Current line is unindented past this current level, so return to get to outside scope
            indent_stack.pop();
            // Check if outer-most scope was exited
            if *indent_stack.last().unwrap() == 0 { 
              indent_stack.pop();
              return Ok((match_token(&tokens2, &Token::TokDedent(*n)).unwrap(), body_contents.to_vec()));
            }
            return Ok((tokens2, body_contents.to_vec()));
          } else {
            return Err(format!("IndentationError: unindent of size {} does not match any outer indentation level", n));
          }
        },
        Some(_) => Err(format!("SyntaxError: not all tokens from previous line were parsed")),
        None => {
          body_contents.push(parsed_line);
          match read_body_line(prev_indent) {
            Ok((next_line, indentation)) => {
              *prev_indent = indentation;
        
              // need to match dedents here since recursive call causes tokens to be parsed
              match lookahead(&next_line) {
        
                // Input is unindented, exit to outer scope
                Some(Token::TokDedent(n)) => {
                  let peek = indent_stack.last().unwrap();
        
                  if *n == *peek {
                    if *peek == 0 { 
                      indent_stack.pop();
                      return Ok((match_token(&next_line, &Token::TokDedent(*n)).unwrap(), body_contents.to_vec()));
                    }
                    return Ok((next_line, body_contents.to_vec()));
                  } else if *n < *peek {
                    return Ok((next_line, body_contents.to_vec()));
                  }
                  return Err(format!("IndentationError: unindent of size {} does not match any outer indentation level", n));
        
                },
                // Line on same level was entered, parse it and continue on this level
                _ => parse_body(&next_line, prev_indent, body_contents, indent_stack)
                // None => {
                //   return Err(format!("Syntax Error: nothing entered"));
                // }
              } 
            },
            Err(e) => Err(e)
          }
        }
      }
    },
    Err(e) => Err(e)
  }
}

fn parse_expr(tokens: &Vec<Token>) -> Result<(Vec<Token>, PyType), String> {
  match parse_and(tokens) {
    Ok((tokens2, and_expr)) => {
      match lookahead(&tokens2) {
        // AndExpr or OrExpr
        Some(Token::TokOr) => {
          match parse_expr(&match_token(&tokens2, &Token::TokOr).unwrap()) {
            Ok((tokens3, or_expr)) => {
              Ok((tokens3, PyType::Expr(Expr::Binop(Op::Or, Box::from(and_expr), Box::from(or_expr)))))
            },
            Err(e) => Err(e)
          }
        },
        // AndExpr
        _ => Ok((tokens2, and_expr))
      }
    },
    Err(e) => Err(e)
  }
}

fn parse_and(tokens: &Vec<Token>) -> Result<(Vec<Token>, PyType), String> {
  match parse_equality(tokens) {
    Ok((tokens2, equality_expr)) => {
      match lookahead(&tokens2) {
        // EqualityExpr and AndExpr
        Some(Token::TokAnd) => {
          match parse_and(&match_token(&tokens2, &Token::TokAnd).unwrap()) {
            Ok((tokens3, and_expr)) => {
              Ok((tokens3, PyType::Expr(Expr::Binop(Op::And, Box::from(equality_expr), Box::from(and_expr)))))
            },
            Err(e) => Err(e)
          }
        },
        // EqualityExpr
        _ => Ok((tokens2, equality_expr))
      }
    },
    Err(e) => Err(e)
  }
}

fn parse_equality(tokens: &Vec<Token>) -> Result<(Vec<Token>, PyType), String> {
  match parse_relational(tokens) {
    Ok((tokens2, relational_expr)) => {
      match lookahead(&tokens2) {
        // RelationalExpr == EqualityExpr
        Some(Token::TokDoubleEqual) => {
          match parse_equality(&match_token(&tokens2, &Token::TokDoubleEqual).unwrap()) {
            Ok((tokens3, equality_expr)) => {
              Ok((tokens3, PyType::Expr(Expr::Binop(Op::Equal, Box::from(relational_expr), Box::from(equality_expr)))))
            },
            Err(e) => Err(e)
          }
        },
        // RelationalExpr != EqualityExpr
        Some(Token::TokNotEqual) => {
          match parse_equality(&match_token(&tokens2, &Token::TokNotEqual).unwrap()) {
            Ok((tokens3, equality_expr)) => {
              Ok((tokens3, PyType::Expr(Expr::Binop(Op::NotEqual, Box::from(relational_expr), Box::from(equality_expr)))))
            },
            Err(e) => Err(e)
          }
        },
        // RelationalExpr
        _ => Ok((tokens2, relational_expr))
      }
    },
    Err(e) => Err(e)
  }
}

fn parse_relational(tokens: &Vec<Token>) -> Result<(Vec<Token>, PyType), String> {
  match parse_additive(tokens) {
    Ok((tokens2, additive_expr)) => {
      match lookahead(&tokens2) {
        // AdditiveExpr < RelationalExpr
        Some(Token::TokLess) => {
          match parse_relational(&match_token(&tokens2, &Token::TokLess).unwrap()) {
            Ok((tokens3, relational_expr)) => {
              Ok((tokens3, PyType::Expr(Expr::Binop(Op::Less, Box::from(additive_expr), Box::from(relational_expr)))))
            },
            Err(e) => Err(e)
          }
        },
        // AdditiveExpr > RelationalExpr
        Some(Token::TokGreater) => {
          match parse_relational(&match_token(&tokens2, &Token::TokGreater).unwrap()) {
            Ok((tokens3, relational_expr)) => {
              Ok((tokens3, PyType::Expr(Expr::Binop(Op::Greater, Box::from(additive_expr), Box::from(relational_expr)))))
            },
            Err(e) => Err(e)
          }
        },
        // AdditiveExpr <= RelationalExpr
        Some(Token::TokLessEqual) => {
          match parse_relational(&match_token(&tokens2, &Token::TokLessEqual).unwrap()) {
            Ok((tokens3, relational_expr)) => {
              Ok((tokens3, PyType::Expr(Expr::Binop(Op::LessEqual, Box::from(additive_expr), Box::from(relational_expr)))))
            },
            Err(e) => Err(e)
          }
        },
        // AdditiveExpr >= RelationalExpr
        Some(Token::TokGreaterEqual) => {
          match parse_relational(&match_token(&tokens2, &Token::TokGreaterEqual).unwrap()) {
            Ok((tokens3, relational_expr)) => {
              Ok((tokens3, PyType::Expr(Expr::Binop(Op::GreaterEqual, Box::from(additive_expr), Box::from(relational_expr)))))
            },
            Err(e) => Err(e)
          }
        },
        // AdditiveExpr
        _ => Ok((tokens2, additive_expr))
      }
    },
    Err(e) => Err(e)
  }
}

fn parse_additive(tokens: &Vec<Token>) -> Result<(Vec<Token>, PyType), String> {
  match parse_multiplicative(tokens) {
    Ok((tokens2, mult_expr)) => {
      match lookahead(&tokens2) {
        // MultExpr + AddExpr
        Some(Token::TokPlus) => {
          match parse_additive(&match_token(&tokens2, &Token::TokPlus).unwrap()) {
            Ok((tokens3, add_expr)) => {
              return Ok((tokens3, PyType::Expr(Expr::Binop(Op::Add, Box::from(mult_expr), Box::from(add_expr)))))
            },
            Err(e) => Err(e)
          }
        },

        // MultExpr - AddExpr
        Some(Token::TokMinus) => {
          match parse_additive(&match_token(&tokens2, &Token::TokMinus).unwrap()) {
            Ok((tokens3, add_expr)) => {
              return Ok((tokens3, PyType::Expr(Expr::Binop(Op::Sub, Box::from(mult_expr), Box::from(add_expr)))))
            },
            Err(e) => Err(e)
          }
        },

        // MultExpr TokUnaryMinus NumericalExpr
        Some(Token::TokUnaryMinus) => {
          match parse_primary(&match_token(&tokens2, &Token::TokUnaryMinus).unwrap()) {
            Ok((tokens3, num_expr)) => {
              return Ok((tokens3, PyType::Expr(Expr::Binop(Op::Sub, Box::from(mult_expr), Box::from(num_expr)))))
            },
            Err(e) => Err(e)
          }
        },

        // MultExpr
        _ => Ok((tokens2, mult_expr))
      }
    }, 
    Err(e) => Err(e)
}
}

fn parse_multiplicative(tokens: &Vec<Token>) -> Result<(Vec<Token>, PyType), String> {
  match parse_unary(tokens) {
    Ok((tokens2, unary_expr)) => {
      match lookahead(&tokens2) {
        // UnaryExpr * MultExpr
        Some(Token::TokMult) => {
          match parse_multiplicative(&match_token(&tokens2, &Token::TokMult).unwrap()) {
            Ok((tokens3, mult_expr)) => {
              return Ok((tokens3, PyType::Expr(Expr::Binop(Op::Mult, Box::from(unary_expr), Box::from(mult_expr)))))
            },
            Err(e) => Err(e)
          }
        },

        // UnaryExpr / MultExpr
        Some(Token::TokDiv) => {
          match parse_multiplicative(&match_token(&tokens2, &Token::TokDiv).unwrap()) {
            Ok((tokens3, mult_expr)) => {
              return Ok((tokens3, PyType::Expr(Expr::Binop(Op::Div, Box::from(unary_expr), Box::from(mult_expr)))))
            },
            Err(e) => Err(e)
          }
        }

        // UnaryExpr
        _ => Ok((tokens2, unary_expr))
      }
    },
    Err(e) => Err(e)
  }
}

fn parse_unary(tokens: &Vec<Token>) -> Result<(Vec<Token>, PyType), String> {
  match lookahead(&tokens) {
    // TokUnaryMinus NumericalExpr
    Some(Token::TokUnaryMinus) => {
      match parse_primary(&match_token(tokens, &Token::TokUnaryMinus).unwrap()) {
        Ok((tokens2, num_expr)) => {
          Ok((tokens2, PyType::Expr(Expr::Binop(Op::Mult, Box::from(PyType::Expr(Expr::Int(-1))), Box::from(num_expr)))))
        },
        Err(e) => Err(e)
      }
    },

    // TokNot NumericalExpr
    Some(Token::TokNot) => {
      match parse_primary(&match_token(tokens, &Token::TokNot).unwrap()) {
        Ok((tokens2, num_expr)) => {
          Ok((tokens2, PyType::Expr(Expr::Not(Box::from(num_expr)))))
        },
        Err(e) => Err(e)
      }
    }

    // NumericalExpr
    _ => parse_primary(tokens)
  }
  
}

fn parse_primary(tokens: &Vec<Token>) -> Result<(Vec<Token>, PyType), String> {
  match lookahead(&tokens) {
    // Int
    Some(Token::TokInt(n)) => {
      Ok((match_token(&tokens, &Token::TokInt(*n)).unwrap(), PyType::Expr(Expr::Int(*n))))
    },

    // Float
    Some(Token::TokFloat(d)) => {
      Ok((match_token(&tokens, &Token::TokFloat(*d)).unwrap(), PyType::Expr(Expr::Float(*d))))
    },

    // String
    Some(Token::TokString(s)) => {
      Ok((match_token(&tokens, &Token::TokString(s.clone())).unwrap(), PyType::Expr(Expr::String(s.clone()))))
    },

    // Bool
    Some(Token::TokBool(b)) => {
      Ok((match_token(&tokens, &Token::TokBool(*b)).unwrap(), PyType::Expr(Expr::Bool(*b))))
    },

    // Var
    Some(Token::TokVar(v)) => {
      Ok((match_token(&tokens, &Token::TokVar(v.clone())).unwrap(), PyType::Expr(Expr::Var(v.clone()))))
    },

    // (OrExpr) or error
    _ => {
      // Match opening parenthesis
      match match_token(&tokens, &Token::TokLParen) {
        Ok(tokens2) => {
          // Parse expression inside parentheses
          match parse_expr(&tokens2) {
            Ok((tokens3, expr)) => {
              // Match closing parenthesis
              match match_token(&tokens3, &Token::TokRParen) {
                Ok(tokens4) => Ok((tokens4, expr)),
                Err(e) => Err(e)
              }
            },
            Err(e) => Err(e)
          }
        }
        Err(e) => Err(e)
      }
    }
  }
}