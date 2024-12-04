use crate::types::{Token, Expr, Op};

pub fn parse(tokens: &Vec<Token>) -> Result<(Vec<Token>, Expr), String> {
  parse_additive(tokens)
}

fn lookahead(tokens: &Vec<Token>) -> Option<&Token> {
  tokens.get(0)
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

    // Bool
    Some(Token::TokBool(b)) => {
      Ok((match_token(&tokens, &Token::TokBool(*b)).unwrap(), Expr::Bool(*b)))
    },

    // (Expr) or error
    _ => {
      // Match opening parenthesis
      match match_token(&tokens, &Token::TokLParen) {
        Ok(tokens2) => {
          // Parse expression inside parentheses
          match parse(&tokens2) {
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