use crate::types::Token;
use regex::Regex;

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
  let mut input = input;

  // Regex Patterns
  let re_whitespace = Regex::new(r"^(\s+)").unwrap();
  let re_pos_int = Regex::new(r"^(\d+)").unwrap();
  let re_neg_int = Regex::new(r"^(-)(\d+)").unwrap();
  let re_pos_float = Regex::new(r"^(\d*\.\d+)").unwrap();
  let re_neg_float = Regex::new(r"^(-)(\d*\.\d+)").unwrap();
  let re_bool = Regex::new(r"^(True|False)").unwrap();
  let re_plus = Regex::new(r"^(\+)").unwrap();
  let re_minus = Regex::new(r"^(-)").unwrap();
  let re_mult = Regex::new(r"^(\*)").unwrap();
  let re_div = Regex::new(r"^(/)").unwrap();
  let re_lparen = Regex::new(r"^(\()").unwrap();
  let re_rparen = Regex::new(r"^(\))").unwrap();
  let re_or = Regex::new(r"^(or)").unwrap();
  let re_and = Regex::new(r"^(and)").unwrap();
  let re_double_equal = Regex::new(r"^(==)").unwrap();
  let re_not_equal = Regex::new(r"^(!=)").unwrap();
  let re_less = Regex::new(r"^(<)").unwrap();
  let re_greater = Regex::new(r"^(>)").unwrap();
  let re_less_equal = Regex::new(r"^(<=)").unwrap();
  let re_greater_equal = Regex::new(r"^(>=)").unwrap();
  let re_string = Regex::new(r#"^(\".*\"|'[^\"]*')"#).unwrap();
  let mut tokens = Vec::new();

  while input.len() > 0 {
    // Whitespace
    if let Some(capture) = re_whitespace.captures(input) {
      let capture_str = capture.get(0).unwrap().as_str();
      input = &input[capture_str.len()..];
    }

    // Non-negative Float
    else if let Some(capture) = re_pos_float.captures(input) {
      let capture_str = capture.get(0).unwrap().as_str();
      tokens.push(Token::TokFloat(capture_str.parse::<f32>().unwrap()));
      input = &input[capture_str.len()..];
    }

    // Negative Float
    else if let Some(capture) = re_neg_float.captures(input) {
      let capture_str = capture.get(2).unwrap().as_str();
      tokens.push(Token::TokUnaryMinus);
      tokens.push(Token::TokFloat(capture_str.parse::<f32>().unwrap()));
      input = &input[(capture_str.len() + 1)..]; // + 1 to account for minus sign
    }

    // Non-negative Int
    else if let Some(capture) = re_pos_int.captures(input) {
      let capture_str = capture.get(0).unwrap().as_str();
      tokens.push(Token::TokInt(capture_str.parse::<i32>().unwrap()));
      input = &input[capture_str.len()..];
    }

    // Negative Int
    else if let Some(capture) = re_neg_int.captures(input) {
      let capture_str = capture.get(2).unwrap().as_str();
      tokens.push(Token::TokUnaryMinus);
      tokens.push(Token::TokInt(capture_str.parse::<i32>().unwrap()));
      input = &input[(capture_str.len() + 1)..]; // + 1 to account for minus sign
    }

    // String
    else if let Some(capture) = re_string.captures(input) {
      let capture_str = capture.get(0).unwrap().as_str();
      let mut chars = capture_str.chars(); // To remove quotes surrounding string
      chars.next();
      chars.next_back();
      tokens.push(Token::TokString(String::from(chars.as_str())));
      input = &input[(capture_str.len())..];
    }

    // Bool
    else if let Some(capture) = re_bool.captures(input) {
      let capture_str = capture.get(0).unwrap().as_str();
      if capture_str == "True" {
        tokens.push(Token::TokBool(true));
      } else {
        tokens.push(Token::TokBool(false));
      }
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

    // Or
    else if let Some(_) = re_or.captures(input) {
      tokens.push(Token::TokOr);
      input = &input[2..];
    }

    // And
    else if let Some(_) = re_and.captures(input) {
      tokens.push(Token::TokAnd);
      input = &input[3..];
    }
    
    // ==
    else if let Some(_) = re_double_equal.captures(input) {
      tokens.push(Token::TokDoubleEqual);
      input = &input[2..];
    }

    // !=
    else if let Some(_) = re_not_equal.captures(input) {
      tokens.push(Token::TokNotEqual);
      input = &input[2..];
    }

    // <=
    else if let Some(_) = re_less_equal.captures(input) {
      tokens.push(Token::TokLessEqual);
      input = &input[2..];
    }

    // >=
    else if let Some(_) = re_greater_equal.captures(input) {
      tokens.push(Token::TokGreaterEqual);
      input = &input[2..];
    }

    // <
    else if let Some(_) = re_less.captures(input) {
      tokens.push(Token::TokLess);
      input = &input[1..];
    }
    // >
    else if let Some(_) = re_greater.captures(input) {
      tokens.push(Token::TokGreater);
      input = &input[1..];
    }

    // Invalid Input
    else {
      return Err(format!("{}{}", "Error lexing input: ", input));
    }
  }

  return Ok(tokens);
}