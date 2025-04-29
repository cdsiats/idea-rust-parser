use crate::{lexer::Lexer, tokens::{Span, Token, TokenKind}};

fn lex(source: &str) -> Vec<Token> {
  Lexer::new(source).tokenize()
}

#[test]
fn test_empty_input() {
  let tokens = lex("");
  assert_eq!(tokens.len(), 1);
  assert_eq!(tokens[0].kind, TokenKind::EOF);
  assert_eq!(tokens[0].span, Span { start: 0, end: 0 });
}

#[test]
fn test_symbols() {
  let tokens = lex("{}()[]!");
  let kinds: Vec<TokenKind> = tokens
    .iter()
    .map(|t| t.kind.clone())
    .collect();
  assert_eq!(kinds, vec![
    TokenKind::LBrace,
    TokenKind::RBrace,
    TokenKind::LParen,
    TokenKind::RParen,
    TokenKind::LBracket,
    TokenKind::RBracket,
    TokenKind::Bang,
    TokenKind::EOF,
]);
}