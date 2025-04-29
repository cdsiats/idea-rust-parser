
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
  pub start: usize,
  pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
  //keywords
  Enum, Model, Type, Prop, Plugin, Use,
  //punctuations
  LBrace, RBrace, LParen, RParen, LBracket, RBracket, Bang,
  //literals and identifiers
  Identifier(String),
  StringLiteral(String),
  Number(f64),
  EOF
}

#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}

