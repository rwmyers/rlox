use crate::{Token, TokenType, to_ascii_chars};

pub struct Scanner {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub source: String,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            source: to_ascii_chars(source),
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();
        if c.is_ascii_alphabetic() {
            return self.identifier();
        }
        if c.is_ascii_digit() {
            return self.number();
        }
        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            },
            '"' => self.string(),
            _ => self.error_token(&format!("Unexpected character: '{}'", c))
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek() == '\0'
    }

    fn peek(&self) -> char {
        self.get_source_char(self.current)
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.get_source_char(self.current + 1)
        }
    }

    fn get_source_char(&self, index: usize) -> char {
        let opt = self.source.chars().nth(index);
        opt.unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    let _ = self.advance();
                },
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return
            }
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // Closing quote
        self.advance();
        self.make_token(TokenType::String)
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let identifier_type = self.identifier_type();
        self.make_token(identifier_type)
    }

    fn identifier_type(&mut self) -> TokenType {
        let c = self.get_source_char(self.start);
        match c {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start <= 1 {
                    TokenType::Identifier
                } else {
                    let c = self.get_source_char(self.start + 1);
                    match c {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier
                    }
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current - self.start <= 1 {
                    TokenType::Identifier
                } else {
                    let c = self.get_source_char(self.start + 1);
                    match c {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier
                    }
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the '.'
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.get_source_char(self.current - 1)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            &self.source[self.start..self.current],
            self.line)
    }

    fn error_token(&self, message: &str) -> Token {
        Token::new(
            TokenType::Error,
            message,
            self.line
        )
    }

    fn check_keyword(&self, start: usize, _length: usize, rest: &str, token_type: TokenType) -> TokenType {
        let actual = &self.source[self.start + start..self.current];
        if actual == rest {
            token_type
        } else {
            TokenType::Identifier
        }
    }
}