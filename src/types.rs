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
    Binop(Op, Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Int(n) => write!(f, "Int({})", n),
            Expr::Float(d) => write!(f, "Float({})", d),
            Expr::Binop(op, left, right) => {
                write!(f, "Binop({}, {}, {})", op, left, right)
            }
        }
    }
}

pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mult => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}
