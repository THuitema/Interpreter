use std::fmt;

#[derive(Debug, PartialEq, Clone)] // Derive PartialEq for comparison
pub enum Token {
    TokInt(i32),
    TokFloat(f32),
    TokUnaryMinus,
    TokPlus,
    TokMinus,
    TokMult,
    TokDiv,
    TokLParen,
    TokRParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::TokInt(n) => write!(f, "TokInt({})", n),
            Token::TokFloat(d) => write!(f, "TokFloat({})", d),
            Token::TokUnaryMinus => write!(f, "TokUnaryMinus"),
            Token::TokPlus => write!(f, "TokPlus"),
            Token::TokMinus => write!(f, "TokMinus"),
            Token::TokMult => write!(f, "TokMult"),
            Token::TokDiv => write!(f, "TokDiv"),
            Token::TokLParen => write!(f, "TokLParen"),
            Token::TokRParen => write!(f, "TokRParen"),
        }
    }
}

pub enum Expr {
  Int(i32),
  Float(f32),
  Binop(Op, Box<Expr>, Box<Expr>)
}

pub enum Op {
  Add,
  Sub,
  Mult,
  Div
}