use crate::types::Token;
use regex::Regex;

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
  let mut input = input;

  // Regex Patterns
  let re_whitespace = Regex::new(r"^(\s+)").unwrap();
  let re_int = Regex::new(r"^(-?\d+)").unwrap();
  let re_float = Regex::new(r"^(-?\d*\.\d+)").unwrap();
  let re_plus = Regex::new(r"^(\+)").unwrap();
  let re_minus = Regex::new(r"^(-)").unwrap();
  let re_mult = Regex::new(r"^(\*)").unwrap();
  let re_div = Regex::new(r"^(/)").unwrap();
  let re_lparen = Regex::new(r"^(\()").unwrap();
  let re_rparen = Regex::new(r"^(\))").unwrap();

  let mut tokens = Vec::new();

  while input.len() > 0 {
    // Whitespace
    if let Some(capture) = re_whitespace.captures(input) {
      let capture_str = capture.get(0).unwrap().as_str();
      input = &input[capture_str.len()..];
    }

    // Float
    else if let Some(capture) = re_float.captures(input) {
      let capture_str = capture.get(0).unwrap().as_str();
      tokens.push(Token::TokFloat(capture_str.parse::<f32>().unwrap()));
      input = &input[capture_str.len()..];
    }

    // Int
    else if let Some(capture) = re_int.captures(input) {
      let capture_str = capture.get(0).unwrap().as_str();
      tokens.push(Token::TokInt(capture_str.parse::<i32>().unwrap()));
      input = &input[capture_str.len()..];
    }

    // Plus
    else if let Some(_) = re_plus.captures(input) {
      tokens.push(Token::TokPlus);
      input = &input[1..];
    }

    // Minus
    else if let Some(_) = re_minus.captures(input) {
      tokens.push(Token::TokMinus);
      input = &input[1..];
    }

    // Mult
    else if let Some(_) = re_mult.captures(input) {
      tokens.push(Token::TokMult);
      input = &input[1..];
    }

    // Div
    else if let Some(_) = re_div.captures(input) {
      tokens.push(Token::TokDiv);
      input = &input[1..];
    }

    // Left Parenthesis
    else if let Some(_) = re_lparen.captures(input) {
      tokens.push(Token::TokLParen);
      input = &input[1..];
    }

    // Right Parenthesis
    else if let Some(_) = re_rparen.captures(input) {
      tokens.push(Token::TokRParen);
      input = &input[1..];
    }

    // Invalid Input
    else {
      return Err(format!("{}{}", "Error lexing input: ", input));
    }
  }

  return Ok(tokens);
}