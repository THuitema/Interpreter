pub enum Token {
  TokInt(i32),
  TokFloat(f32),
  TokPlus,
  TokMinus,
  TokMult,
  TokDiv,
  TokLParen,
  TokRParen
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