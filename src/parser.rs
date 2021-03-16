use sea_query::{Token, Tokenizer};

pub struct Parser {
    pub tokens: Tokenizer,
    pub tok: Option<Token>,
}

impl Parser {
    pub fn new(string: &str) -> Self {
        Self {
            tokens: Tokenizer::new(string),
            tok: None,
        }
    }

    pub fn curr(&mut self) -> Option<&Token> {
        if self.tok.is_some() {
            self.tok.as_ref()
        } else {
            self.next()
        }
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.tok = None;

        if let Some(tok) = self.tokens.next() {
            if tok.is_space() {
                if let Some(tok) = self.tokens.next() {
                    self.tok = Some(tok);
                }
            } else {
                self.tok = Some(tok);
            }
        }
        self.tok.as_ref()
    }
}