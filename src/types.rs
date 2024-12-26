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
    TokVar(String),
    TokAssign,
    TokIf,
    TokElif,
    TokElse,
    TokColon,
    TokIndent(i32),
    TokDedent(i32),
    TokNot
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::TokInt(n) => write!(f, "TokInt({})", n),
            Token::TokFloat(d) => write!(f, "TokFloat({})", d),
            Token::TokBool(b) => write!(f, "TokBool({})", b),
            Token::TokString(s) => write!(f, "TokString(\"{}\")", s),
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
            Token::TokVar(s) => write!(f, "TokVar({})", s),
            Token::TokAssign => write!(f, "TokAssign"),
            Token::TokIf => write!(f, "TokIf"),
            Token::TokElif => write!(f, "TokElif"),
            Token::TokElse => write!(f, "TokElse"),
            Token::TokColon => write!(f, "TokColon"),
            Token::TokIndent(n) => write!(f, "TokIndent({})", n),
            Token::TokDedent(n) => write!(f, "TokDedent({})", n),
            Token::TokNot => write!(f, "TokNot"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PyType {
    Stmt(Stmt),
    Expr(Expr)
}

impl fmt::Display for PyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PyType::Stmt(s) => write!(f, "{}", s), // Ok(())
            PyType::Expr(e) => write!(f, "{}", e)
        }
    }

}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Var(String),
    Binop(Op, Box<PyType>, Box<PyType>),
    Not(Box<PyType>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    If(Box<PyType>, Vec<PyType>, Option<Vec<PyType>>), // condition, body if true, else body
    VarAssign(String, Box<PyType>),
    None
}

#[derive(Clone, Debug, PartialEq)]
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

impl Expr {
    pub fn to_bool(&self) -> Result<bool, String> {
        match self {
            Expr::Bool(b) => Ok(*b),
            Expr::Int(n) => if *n == 0 {Ok(false)} else {Ok(true)},
            Expr::Float(n) => if *n == 0.0 {Ok(false)} else {Ok(true)},
            _ => Err(format!("TypeError: Cannot convert type to bool"))
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Int(n) => write!(f, "{}", n),
            Expr::Float(d) => write!(f, "{}", d),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::Bool(b) => if *b {write!(f, "True")} else {write!(f, "False")},
            Expr::Var(v) => write!(f, "{}", v),
            Expr::Binop(op, left, right) => {
                write!(f, "{} {} {}", left, op, right)
            },
            Expr::Not(e) => write!(f, "Not({})", e)
        }
    }
}



impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::VarAssign(v, e) => write!(f, "{} = {}", v, e),
            Stmt::If(condition, body, else_body) => {
                write!(f, "If({}, [", condition);

                for expr in body {
                    write!(f, "{}, ", expr);
                }
                write!(f, "])");
                if let Some(else_body_list) = else_body {
                    write!(f, " Else [");
                    for expr in else_body_list {
                        write!(f, "{}, ", expr);
                    }
                    write!(f, "]")
                } else {
                    write!(f, "")
                } 
            },
            Stmt::None => write!(f, "")
        }
    }
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

pub type Environment = Vec<(String, PyType)>;
pub fn print_env(env: &Environment) {
    print!("Env: [");
    for (v, e) in env {
        print!("{} = {}, ", v, e);
    }
    println!("]");
}
