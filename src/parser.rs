use crate::lexer::tokenize;
use crate::types::{Token, Expr, Op};
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

pub fn parse(tokens: &Vec<Token>, prev_indent: &mut i32, indent_stack: &mut Vec<i32>) -> Result<(Vec<Token>, Expr), String> {
  match (lookahead(tokens), lookahead_at(tokens, 1)) {
    // AssignStatement
    (Some(Token::TokVar(v)), Some(Token::TokAssign)) => parse_assign(&match_token(&tokens, &Token::TokVar(v.clone())).unwrap(), v),
    
    // IfStatement
    (Some(Token::TokIf), _) => parse_if(&match_token(&tokens, &Token::TokIf).unwrap(), prev_indent, indent_stack),
    
    // Expr
    _ => parse_expr(tokens)
  }
}

fn parse_assign(tokens: &Vec<Token>, var_name: &String) -> Result<(Vec<Token>, Expr), String> {
  match parse_expr(&match_token(&tokens, &Token::TokAssign).unwrap()) {
    Ok((tokens2, e)) => Ok((tokens2, Expr::VarAssign(var_name.clone(), Box::from(e)))),
    Err(e) => Err(e)
  }
}

fn parse_if(tokens: &Vec<Token>, prev_indent: &mut i32, indent_stack: &mut Vec<i32>) -> Result<(Vec<Token>, Expr), String> {
  // println!("Prev indent: {}", prev_indent);
  indent_stack.push(*prev_indent);
  println!("Parsing if");
  match parse_expr(tokens) {
    // Condition of if statement
    Ok((tokens2, condition)) => {
      // match colon token
      match match_token(&tokens2, &Token::TokColon) {
        Ok(tokens3) if tokens3.is_empty() => {
          // Read first line of closure under if statement and tokenize
          match read_body_line(prev_indent) {
            Ok((body_tokens, indentation)) => {
              *prev_indent = indentation;
              match lookahead(&body_tokens) {
                // Get indent for first line of body
                Some(Token::TokIndent(n)) => {
                  // Parsing body of if statement
                  let mut body_contents = Vec::<Expr>::new();
                  match parse_body(&match_token(&body_tokens, &Token::TokIndent(*n)).unwrap(), prev_indent, &mut body_contents, indent_stack) {
                    Ok((tokens4, _)) => {
                      print!("Tokens: ");
                      for i in &tokens4 {
                        print!(" {},", i);
                      }
                      println!("");
                      Ok((tokens4, Expr::If(Box::from(condition), body_contents)))
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

fn parse_body(tokens: &Vec<Token>, prev_indent: &mut i32, body_contents: &mut Vec<Expr>, indent_stack: &mut Vec<i32>) -> Result<(Vec<Token>, Vec<Expr>), String> {
  print!("INDENT STACK:");
  for i in indent_stack.clone() {
    print!(" {},", i);
  }
  println!("");
  match lookahead(tokens) {
    Some(Token::TokDedent(n)) => {
      // Remove the while loop, make the popping part of the recursion
      if let Some(peek) = indent_stack.last() {
        if *n == *peek {
          println!("n == peek");
          if *n == 0 { // unindenting to no indent -- i.e. exiting all closures
            return Ok((match_token(&tokens, &Token::TokDedent(*n)).unwrap(), body_contents.to_vec()))
          }
          
          return Ok((tokens.to_vec(), body_contents.to_vec())) // match_token(&tokens, &Token::TokDedent(*n)).unwrap()
        } else if *n < *peek {
          // this may perform the same action as n == peek, needs to exit multiple levels of indents
          // miiight need to call parse_body here, but i dont think so
          println!("n < peek");
          indent_stack.pop();
          // may need to parse_body instead of Ok
          
          return Ok((tokens.to_vec(), body_contents.to_vec()))
          // return parse_body(tokens, prev_indent, body_contents, indent_stack);
        } 
      }
      return Err(format!("IndentationError: unindent of size {} does not match any outer indentation level", n));
    },
    _ => {
      // Parse current line

      //***********
      // Getting parsing error when trying to unindent multiple levels in one line.
      // Pretty sure problem is Dedent token doesn't get removed, so it tries to parse it

      match parse(tokens, prev_indent, indent_stack) {
        Ok((tokens2, expr)) => { // if tokens2.is_empty() 
          
          match lookahead(&tokens2) {
            Some(Token::TokDedent(n)) => {
              body_contents.push(expr);
              let peek = indent_stack.last().unwrap();
              if *n == *peek { // Are on current level of this dedent, so we can parse the line
                indent_stack.pop();
                // parse rest of tokens
                print!("3 Tokens: ");
                for i in &tokens2 {
                  print!(" {},", i);
                }
                println!("");
                if indent_stack.is_empty() {
                  return Ok((match_token(&tokens2, &Token::TokDedent(*n)).unwrap(), body_contents.to_vec()));
                }
                // println!("HERE 2");
                return parse_body(&match_token(&tokens2, &Token::TokDedent(*n)).unwrap(), prev_indent, body_contents, indent_stack);
              }
              
              Ok((tokens2, body_contents.to_vec()))
            },
            Some(_) => Err(format!("SyntaxError: not all tokens from previous line were parsed")),
            None => {
              body_contents.push(expr);

              // println!("HERE");
              match read_body_line(prev_indent) {
                Ok((next_line, indentation)) => {
                  print!("2 Tokens: ");
                  for i in &next_line {
                    print!(" {},", i);
                  }
                  println!("");
                  *prev_indent = indentation;
                  parse_body(&next_line, prev_indent, body_contents, indent_stack)
                },
                Err(e) => Err(e)
              }
            }
          }
        },
        Err(e) => Err(e),
      }
    }
    
  }
}

fn parse_expr(tokens: &Vec<Token>) -> Result<(Vec<Token>, Expr), String> {
  match parse_and(tokens) {
    Ok((tokens2, and_expr)) => {
      match lookahead(&tokens2) {
        // AndExpr or OrExpr
        Some(Token::TokOr) => {
          match parse_expr(&match_token(&tokens2, &Token::TokOr).unwrap()) {
            Ok((tokens3, or_expr)) => {
              Ok((tokens3, Expr::Binop(Op::Or, Box::from(and_expr), Box::from(or_expr))))
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

fn parse_and(tokens: &Vec<Token>) -> Result<(Vec<Token>, Expr), String> {
  match parse_equality(tokens) {
    Ok((tokens2, equality_expr)) => {
      match lookahead(&tokens2) {
        // EqualityExpr and AndExpr
        Some(Token::TokAnd) => {
          match parse_and(&match_token(&tokens2, &Token::TokAnd).unwrap()) {
            Ok((tokens3, and_expr)) => {
              Ok((tokens3, Expr::Binop(Op::And, Box::from(equality_expr), Box::from(and_expr))))
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

fn parse_equality(tokens: &Vec<Token>) -> Result<(Vec<Token>, Expr), String> {
  match parse_relational(tokens) {
    Ok((tokens2, relational_expr)) => {
      match lookahead(&tokens2) {
        // RelationalExpr == EqualityExpr
        Some(Token::TokDoubleEqual) => {
          match parse_equality(&match_token(&tokens2, &Token::TokDoubleEqual).unwrap()) {
            Ok((tokens3, equality_expr)) => {
              Ok((tokens3, Expr::Binop(Op::Equal, Box::from(relational_expr), Box::from(equality_expr))))
            },
            Err(e) => Err(e)
          }
        },
        // RelationalExpr != EqualityExpr
        Some(Token::TokNotEqual) => {
          match parse_equality(&match_token(&tokens2, &Token::TokNotEqual).unwrap()) {
            Ok((tokens3, equality_expr)) => {
              Ok((tokens3, Expr::Binop(Op::NotEqual, Box::from(relational_expr), Box::from(equality_expr))))
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

fn parse_relational(tokens: &Vec<Token>) -> Result<(Vec<Token>, Expr), String> {
  match parse_additive(tokens) {
    Ok((tokens2, additive_expr)) => {
      match lookahead(&tokens2) {
        // AdditiveExpr < RelationalExpr
        Some(Token::TokLess) => {
          match parse_relational(&match_token(&tokens2, &Token::TokLess).unwrap()) {
            Ok((tokens3, relational_expr)) => {
              Ok((tokens3, Expr::Binop(Op::Less, Box::from(additive_expr), Box::from(relational_expr))))
            },
            Err(e) => Err(e)
          }
        },
        // AdditiveExpr > RelationalExpr
        Some(Token::TokGreater) => {
          match parse_relational(&match_token(&tokens2, &Token::TokGreater).unwrap()) {
            Ok((tokens3, relational_expr)) => {
              Ok((tokens3, Expr::Binop(Op::Greater, Box::from(additive_expr), Box::from(relational_expr))))
            },
            Err(e) => Err(e)
          }
        },
        // AdditiveExpr <= RelationalExpr
        Some(Token::TokLessEqual) => {
          match parse_relational(&match_token(&tokens2, &Token::TokLessEqual).unwrap()) {
            Ok((tokens3, relational_expr)) => {
              Ok((tokens3, Expr::Binop(Op::LessEqual, Box::from(additive_expr), Box::from(relational_expr))))
            },
            Err(e) => Err(e)
          }
        },
        // AdditiveExpr >= RelationalExpr
        Some(Token::TokGreaterEqual) => {
          match parse_relational(&match_token(&tokens2, &Token::TokGreaterEqual).unwrap()) {
            Ok((tokens3, relational_expr)) => {
              Ok((tokens3, Expr::Binop(Op::GreaterEqual, Box::from(additive_expr), Box::from(relational_expr))))
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

fn parse_additive(tokens: &Vec<Token>) -> Result<(Vec<Token>, Expr), String> {
  match parse_multiplicative(tokens) {
    Ok((tokens2, mult_expr)) => {
      match lookahead(&tokens2) {
        // MultExpr + AddExpr
        Some(Token::TokPlus) => {
          match parse_additive(&match_token(&tokens2, &Token::TokPlus).unwrap()) {
            Ok((tokens3, add_expr)) => {
              return Ok((tokens3, Expr::Binop(Op::Add, Box::from(mult_expr), Box::from(add_expr))))
            },
            Err(e) => Err(e)
          }
        },

        // MultExpr - AddExpr
        Some(Token::TokMinus) => {
          match parse_additive(&match_token(&tokens2, &Token::TokMinus).unwrap()) {
            Ok((tokens3, add_expr)) => {
              return Ok((tokens3, Expr::Binop(Op::Sub, Box::from(mult_expr), Box::from(add_expr))))
            },
            Err(e) => Err(e)
          }
        },

        // MultExpr TokUnaryMinus NumericalExpr
        Some(Token::TokUnaryMinus) => {
          match parse_primary(&match_token(&tokens2, &Token::TokUnaryMinus).unwrap()) {
            Ok((tokens3, num_expr)) => {
              return Ok((tokens3, Expr::Binop(Op::Sub, Box::from(mult_expr), Box::from(num_expr))))
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

fn parse_multiplicative(tokens: &Vec<Token>) -> Result<(Vec<Token>, Expr), String> {
  match parse_unary(tokens) {
    Ok((tokens2, unary_expr)) => {
      match lookahead(&tokens2) {
        // UnaryExpr * MultExpr
        Some(Token::TokMult) => {
          match parse_multiplicative(&match_token(&tokens2, &Token::TokMult).unwrap()) {
            Ok((tokens3, mult_expr)) => {
              return Ok((tokens3, Expr::Binop(Op::Mult, Box::from(unary_expr), Box::from(mult_expr))))
            },
            Err(e) => Err(e)
          }
        },

        // UnaryExpr / MultExpr
        Some(Token::TokDiv) => {
          match parse_multiplicative(&match_token(&tokens2, &Token::TokDiv).unwrap()) {
            Ok((tokens3, mult_expr)) => {
              return Ok((tokens3, Expr::Binop(Op::Div, Box::from(unary_expr), Box::from(mult_expr))))
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

fn parse_unary(tokens: &Vec<Token>) -> Result<(Vec<Token>, Expr), String> {
  match lookahead(&tokens) {
    // TokUnaryMinus NumericalExpr
    Some(Token::TokUnaryMinus) => {
      match parse_primary(&match_token(&tokens, &Token::TokUnaryMinus).unwrap()) {
        Ok((tokens2, num_expr)) => {
          Ok((tokens2, Expr::Binop(Op::Mult, Box::from(Expr::Int(-1)), Box::from(num_expr))))
        },
        Err(e) => Err(e)
      }
    },

    // NumericalExpr
    _ => parse_primary(tokens)
  }
  
}

fn parse_primary(tokens: &Vec<Token>) -> Result<(Vec<Token>, Expr), String> {
  match lookahead(&tokens) {
    // Int
    Some(Token::TokInt(n)) => {
      Ok((match_token(&tokens, &Token::TokInt(*n)).unwrap(), Expr::Int(*n)))
    },

    // Float
    Some(Token::TokFloat(d)) => {
      Ok((match_token(&tokens, &Token::TokFloat(*d)).unwrap(), Expr::Float(*d)))
    },

    // String
    Some(Token::TokString(s)) => {
      Ok((match_token(&tokens, &Token::TokString(s.clone())).unwrap(), Expr::String(s.clone())))
    },

    // Bool
    Some(Token::TokBool(b)) => {
      Ok((match_token(&tokens, &Token::TokBool(*b)).unwrap(), Expr::Bool(*b)))
    },

    // Var
    Some(Token::TokVar(v)) => {
      Ok((match_token(&tokens, &Token::TokVar(v.clone())).unwrap(), Expr::Var(v.clone())))
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