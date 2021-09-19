// std

use std::{convert::TryInto, unimplemented};

// local
mod lex;
use lex::{LexError, *};

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    start: usize,
    index: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &str) -> Lexer {
        Lexer {
            source,
            start: 0,
            index: 0,
            line: 0,
        }
    }

    // returns current character without advancing lexer
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.index)
    }

    // returns the nth character after index without advancing lexer
    fn peek_nth(&self, idx: usize) -> Option<char> {
        self.source.chars().nth(self.index + idx)
    }

    fn peek_span(&self, idx: usize) -> &String {
        &self
            .source
            .chars()
            .enumerate()
            .filter(|(idx, _)| {
                idx >= &self.index && idx.clone() <= self.index.clone() + idx.clone()
            })
            .map(|(_, ch)| ch.to_owned())
            .collect::<String>()
    }

    // returns next character and advances lexer by one
    fn next(&mut self) -> Option<char> {
        if !self.is_ended() {
            self.index += 1;
        }

        self.peek()
    }

    fn next_nth(&mut self, idx: usize) -> Option<char> {
        if !self.is_ended() {
            self.index += idx;
        }

        self.peek_nth(idx)
    }

    fn is_ended(&self) -> bool {
        self.index > self.source.len()
    }

    /// creates a span and updates lexer placing
    fn span(&mut self) -> (usize, usize) {
        let s = (self.start, self.index);
        self.start = self.index;

        s
    }

    fn advance_line(&mut self) {
        self.start = 0;
        self.index = 0;
        self.line += 1;
    }

    fn create_token(&mut self, token_type: Token) -> Tok<'a> {
        Tok {
            token_type,
            span: self.span(),
            source: self.source,
        }
    }

    fn to_next(&mut self, character: &str) -> (String, usize) {
        let mut span = String::new();
        let mut idx: usize = 0;
        while let Some(v) = self.peek_nth(character.len()) {
            println!("character: {}, v: {}\n\n\n", character, v);
            let v = v.to_string();
            if v == character {
                break;
            } else {
                idx += 1;
                span.push_str(&v);

                self.next();
            }
        }

        (span, idx)
    }

    fn to_next_token(&mut self) -> String {
        let mut span = String::new();
        while let Some(v) = self.peek() {
            match v {
                '*' | '_' | '~' | '^' | '#' => break,
                _ => span.push(v),
            }
        }

        span
    }
}

pub fn tokenise(src: &str) -> Result<Vec<Tok>, LexError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(src);

    while !lexer.is_ended() {
        let c = lexer.peek();
        lexer.next();

        match c {
            Some(c) => {
                let tok = match c {
                    '*' => match lexer.peek() {
                        Some(c) if c == '*' => {
                            let (txt, idx) = lexer.to_next("**");
                            Token::Bold(txt)
                        }
                        Some(_) => {
                            let (txt, idx) = lexer.to_next("*");
                            Token::Italics(txt)
                        }
                        None => Token::EOI,
                    },
                    '/' => match lexer.peek() {
                        Some(c) if c == '/' => {
                            if let Some(c) = lexer.peek_nth(1) {
                                match c {
                                    _ => {
                                        lexer.advance_line();
                                        Token::Comment
                                    }
                                }
                            } else {
                                Token::EOI
                            }
                        }
                        Some(_) => Token::Text(c.to_string()),
                        None => Token::EOI,
                    },
                    '\t' | '\r' | ' ' => Token::Whitespace,
                    '\n' => {
                        lexer.advance_line();
                        Token::Whitespace
                    }
                    _ => Token::Text(String::from(c)),
                };

                tokens.push(lexer.create_token(tok))
            }
            None => tokens.push(lexer.create_token(Token::EOI)),
        }
    }

    Ok(tokens)
}

/// Tests for lexing grammar
#[cfg(test)]
mod lex_tests {
    use super::*;

    #[test]
    fn verify_paragraphs() {
        let t = tokenise("This is a paragraph. **Bold text.** *Italics text.*").unwrap();
        println!("{:#?}", t);
    }

    #[test]
    fn verify_headings() {
        let t = tokenise("# This is a top level heading").unwrap();
        assert_eq!(t.len(), 1);
        // assert_eq!(matches!(t.get(0).unwrap().token_type, Token::Heading));
    }

    #[test]
    fn verify_tables() {}

    #[test]
    fn verify_unordered_list() {}

    #[test]
    fn verify_ordered_list() {}

    #[test]
    fn verify_checked_list_item() {}

    #[test]
    fn verify_fenced_code() {}

    #[test]
    fn verify_footnotes() {}
}
