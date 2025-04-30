use std::{iter::Peekable, str::Chars};

use crate::tokens::{Span, Token, TokenKind};

#[allow(dead_code)]
pub struct Lexer<'a> {
  input: Peekable<Chars<'a>>,
  position: usize
}
#[allow(dead_code)]
impl<'a> Lexer<'a> {
  pub fn new(src: &'a str) -> Self {
    Lexer { input: src.chars().peekable(), position: 0 }
  }

  fn consume(&mut self) -> Option<char> {
    let ch = self.input.next()?;
    self.position += ch.len_utf8();
    Some(ch)
  }

  fn peek(&mut self) -> Option<&char> {
    self.input.peek()
  }

  pub fn tokenize(mut self) -> Vec<Token> {
    let mut tokens = Vec::new();
    while let Some(&ch) = self.peek() {
      let start = self.position;
      match ch {
        c if c.is_whitespace() => { self.consume(); continue; },
        '{' => { 
          self.consume(); 
          tokens.push(Token { 
            kind: TokenKind::LBrace, 
            span: Span { start, end: self.position } }); 
        },
        '}' => {
          self.consume();
          tokens.push(Token { 
            kind: TokenKind::RBrace, 
            span: Span { start, end: self.position } });
        },
        '(' => {
          self.consume();
          tokens.push(Token { 
            kind: TokenKind::LParen, 
            span: Span { start, end: self.position } })
        },
        ')' => {
          self.consume();
          tokens.push(Token { 
            kind: TokenKind::RParen, 
            span: Span { start, end: self.position } });
        },
        '[' => {
          self.consume();
          tokens.push(Token { 
            kind: TokenKind::LBracket, 
            span: Span { start, end: self.position } });
        },
        ']' => {
          self.consume();
          tokens.push(Token { 
            kind: TokenKind::RBracket, 
            span: Span { start, end: self.position } });
        },
        '!' => {
          self.consume();
          tokens.push(Token { 
            kind: TokenKind::Bang, 
            span: Span { start, end: self.position } });
        },
        '"' => {
          self.consume();
          let mut literal = String::new();
          while let Some(&c) = self.peek() {
            if c == '"' { self.consume(); break; }
            literal.push(c);
            self.consume();
          }
          tokens.push(Token { 
            kind: TokenKind::StringLiteral(literal), 
            span: Span { start, end: self.position } });
        },
        '@' => {
          self.consume();
          let mut attr = String::new();
          attr.push('@');
          while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == '-' {
              attr.push(c);
              self.consume();
            } else { break; }
          }
          tokens.push(Token { 
            kind: TokenKind::AttributeIdentifier(attr.into()), 
            span: Span { start, end: self.position } 
          });
        },
        '/' => {
          self.consume();
          todo!("Implement logic for skipping comments")
        },
        c if c.is_digit(10) || c == '-' => {
          let mut num_str = String::new();
          if c == '-' { num_str.push('-'); self.consume(); }
          while let Some(&ch2) = self.peek() {
            if ch2.is_digit(10) || ch2 == '.' { num_str.push(ch2); self.consume(); }
            else { break; }
          }
          let value = num_str.parse().unwrap_or(0.0);
          tokens.push(Token { 
            kind: TokenKind::Number(value), 
            span: Span { start, end: self.position } });
        },
        c if c.is_alphabetic() || c == '_' => {
          let mut ident = String::new();
          ident.push(c);
          self.consume();
          while let Some(&c2) = self.peek() {
              if c2.is_alphanumeric() || c2 == '_' { ident.push(c2); self.consume(); }
              else { break; }
          }
          // Check keywords
          let kind = match ident.as_str() {
              "enum" => TokenKind::Enum,
              "type" => TokenKind::Type,
              "model" => TokenKind::Model,
              "prop" => TokenKind::Prop,
              "plugin" => TokenKind::Plugin,
              "use" => TokenKind::Use,
              "true" => TokenKind::Boolean(true),
              "false" => TokenKind::Boolean(false),
              _ => TokenKind::Identifier(ident.clone()),
          };
          tokens.push(Token { kind, span: Span { start, end: self.position } });
      }
        _ => { self.consume(); }
      }
    }
    tokens.push(Token { kind: TokenKind::EOF, span: Span { start: self.position, end: self.position } });
    tokens
  }
}