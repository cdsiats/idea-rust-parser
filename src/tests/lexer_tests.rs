use crate::{lexer::Lexer, tokens::{self, Span, Token, TokenKind}};

fn lex(source: &str) -> Vec<Token> {
  Lexer::new(source).tokenize()
}

#[cfg(test)]
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

#[test]
fn test_positive_numbers() {
  let tokens = lex("4 44 4.44");
  assert_eq!(tokens[0].kind, TokenKind::Number(4.0));
  assert_eq!(tokens[0].span, Span { start: 0, end: 1 });
  assert_eq!(tokens[1].kind, TokenKind::Number(44.0));
  assert_eq!(tokens[1].span, Span { start: 2, end: 4 });
  assert_eq!(tokens[2].kind, TokenKind::Number(4.44));
  assert_eq!(tokens[2].span, Span { start: 5, end: 9 });
}

#[test]
fn test_negative_numbers() {
  let tokens = lex("-4 -44 -4.44");
  assert_eq!(tokens[0].kind, TokenKind::Number(-4.0));
  assert_eq!(tokens[0].span, Span { start: 0, end: 2 });
  assert_eq!(tokens[1].kind, TokenKind::Number(-44.0));
  assert_eq!(tokens[1].span, Span { start: 3, end: 6 });
  assert_eq!(tokens[2].kind, TokenKind::Number(-4.44));
  assert_eq!(tokens[2].span, Span { start: 7, end: 12 });
}

#[test]
fn test_identifier() {
  let tokens = lex("enum Foo");
  assert_eq!(tokens.len(), 3);
  assert_eq!(tokens[0].kind, TokenKind::Enum);
  assert_eq!(tokens[0].span, Span { start: 0, end: 4 });
  assert_eq!(tokens[1].kind, TokenKind::Identifier(String::from("Foo")));
  assert_eq!(tokens[1].span, Span { start: 5, end: 8 });
}

#[test]
fn test_string_literals() {
  let tokens = lex("\"hello\"");
  assert_eq!(tokens[0].kind, TokenKind::StringLiteral(String::from("hello")));
  assert_eq!(tokens[0].span, Span { start: 0, end: 7 });
}

#[test]
fn test_model_keyword() {
  let tokens = lex("model");
  assert_eq!(tokens.len(), 2);
  assert_eq!(tokens[0].kind, TokenKind::Model);
  assert_eq!(tokens[0].span, Span { start: 0, end: 5 });
}

#[test]
fn test_enum_keyword() {
  let tokens = lex("enum");
  assert_eq!(tokens.len(), 2);
  assert_eq!(tokens[0].kind, TokenKind::Enum);
  assert_eq!(tokens[0].span, Span { start: 0, end: 4 });
}

#[test]
fn test_type_keyword() {
  let tokens = lex("type");
  assert_eq!(tokens.len(), 2);
  assert_eq!(tokens[0].kind, TokenKind::Type);
  assert_eq!(tokens[0].span, Span { start: 0, end: 4 });
}

#[test]
fn test_plugin_keyword() {
  let tokens = lex("plugin");
  assert_eq!(tokens.len(), 2);
  assert_eq!(tokens[0].kind, TokenKind::Plugin);
  assert_eq!(tokens[0].span, Span { start: 0, end: 6 });
}

#[test]
fn test_use_keyword() {
  let tokens = lex("use");
  assert_eq!(tokens.len(), 2);
  assert_eq!(tokens[0].kind, TokenKind::Use);
  assert_eq!(tokens[0].span, Span { start: 0, end: 3 });
}

#[test]
fn test_attribute_identifier() {
  let tokens = lex("@label");
  assert_eq!(tokens.len(), 2);
  assert_eq!(tokens[0].kind, TokenKind::AttributeIdentifier("@label".into()));
  assert_eq!(tokens[0].span, Span { start: 0, end: 6 });
}

#[test]
fn test_true() {
  let tokens = lex("true");
  assert_eq!(tokens.len(), 2);
  assert_eq!(tokens[0].kind, TokenKind::Boolean(true));
  assert_eq!(tokens[0].span, Span { start: 0, end: 4 });
}

#[test]
fn test_false() {
  let tokens = lex("false");
  assert_eq!(tokens.len(), 2);
  assert_eq!(tokens[0].kind, TokenKind::Boolean(false));
  assert_eq!(tokens[0].span, Span { start: 0, end: 5 });
}

#[test]
fn test_null() {
  let tokens = lex("null");
  assert_eq!(tokens.len(), 2);
  assert_eq!(tokens[0].kind, TokenKind::Null);
  assert_eq!(tokens[0].span, Span { start: 0, end: 4 });
}