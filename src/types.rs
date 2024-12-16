use std::fmt;

#[derive(Debug, PartialEq, Clone)] 

// TODO: create enum for Error codes/messages

pub enum Token {
    TokInt(i32),
    TokFloat(f32),
    TokBool(bool),
    TokString(String),
    TokUnaryMinus,
    TokPlus,
    TokMinus,
    TokMult,
    TokDiv,
    TokLParen,
    TokRParen,
    TokOr,
    TokAnd,
    TokDoubleEqual,
    TokNotEqual,
    TokLess,
    TokGreater,
    TokLessEqual,
    TokGreaterEqual,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::TokInt(n) => write!(f, "TokInt({})", n),
            Token::TokFloat(d) => write!(f, "TokFloat({})", d),
            Token::TokBool(b) => write!(f, "TokBool({})", b),
            Token::TokString(s) => write!(f, "TokString('{}')", s),
            Token::TokUnaryMinus => write!(f, "TokUnaryMinus"),
            Token::TokPlus => write!(f, "TokPlus"),
            Token::TokMinus => write!(f, "TokMinus"),
            Token::TokMult => write!(f, "TokMult"),
            Token::TokDiv => write!(f, "TokDiv"),
            Token::TokLParen => write!(f, "TokLParen"),
            Token::TokRParen => write!(f, "TokRParen"),
            Token::TokOr => write!(f, "TokOr"),
            Token::TokAnd => write!(f, "TokAnd"),
            Token::TokDoubleEqual => write!(f, "TokDoubleEqual"),
            Token::TokNotEqual => write!(f, "TokNotEqual"),
            Token::TokLess => write!(f, "TokLess"),
            Token::TokGreater => write!(f, "TokGreater"),
            Token::TokLessEqual => write!(f, "TokLessEqual"),
            Token::TokGreaterEqual => write!(f, "TokGreaterEqual"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Binop(Op, Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Int(n) => write!(f, "{}", n),
            Expr::Float(d) => write!(f, "{}", d),
            Expr::String(s) => write!(f, "'{}'", s),
            Expr::Bool(b) => if *b {write!(f, "True")} else {write!(f, "False")},
            Expr::Binop(op, left, right) => {
                write!(f, "Binop({}, {}, {})", op, left, right)
            }
        }
    }
}


#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
    Or,
    And,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mult => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Or => write!(f, "or"),
            Op::And => write!(f, "and"),
            Op::Equal => write!(f, "=="),
            Op::NotEqual => write!(f, "!="),
            Op::Less => write!(f, "<"),
            Op::Greater => write!(f, ">"),
            Op::LessEqual => write!(f, "<="),
            Op::GreaterEqual => write!(f, ">="),
        }
    }
}
