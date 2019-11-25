use super::Token;
use super::TokenType;

use crate::traits::LexerTrait;

/// Lexer
/// 
pub struct Lexer {
    source: String,
    curr_line: u32,

    curr_ch: Option<char>,
    peek_ch: Option<char>,

    curr_tok: Token,
    peek_tok: Token 
}

impl Lexer {

    pub fn new(source: String) -> Self {
        let mut lex = Lexer {
            source: source,
            curr_line: 1,

            curr_ch: None,
            peek_ch: None,

            curr_tok: Token::new(TokenType::Illegal, 0),
            peek_tok: Token::new(TokenType::Illegal, 0),
        };
        lex.next_char();
        lex.next();
        lex
    }

    fn next_char(&mut self) -> Option<char> {
        if self.source.len() == 0 {
            self.curr_ch = self.peek_ch;
            self.peek_ch = None;
        } else {
            self.curr_ch = self.peek_ch;
            self.peek_ch = Some(self.source.remove(0));
        }
        self.curr_ch
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.next_char() {
            // Operators
            Some('+') => self.create_token(TokenType::Plus),
            Some('-') => self.create_token(TokenType::Minus),
            Some('/') => self.create_token(TokenType::Divide),

            Some('*') => self.get_peek_token('*', TokenType::Star, TokenType::DoubleStar),
            Some('!') => self.get_peek_token('=', TokenType::Not, TokenType::NotEqual),
            Some('=') => self.get_peek_token('=', TokenType::Assing, TokenType::Equal),

            Some('>') => self.create_token(TokenType::GraterThan),
            Some('<') => self.create_token(TokenType::LessThan),

            Some('.') => self.create_token(TokenType::Dot),

            // Delimiters
            Some(',') => self.create_token(TokenType::Comma),
            Some(';') => self.create_token(TokenType::Semicolon),
            Some(':') => self.get_peek_token(':', TokenType::Colon, TokenType::DoubleColon),

            Some('(') => self.create_token(TokenType::OpenRoundBracket),
            Some(')') => self.create_token(TokenType::CloseRoundBracket),

            Some('{') => self.create_token(TokenType::OpenCurlyBracket),
            Some('}') => self.create_token(TokenType::CloseCurlyBracket),

            Some('[') => self.create_token(TokenType::OpenSquareBracket),
            Some(']') => self.create_token(TokenType::CloseSquareBracket),

            // Keywords + Identifiers + Literals
            Some(ch) => {
                if ch.is_ascii_alphabetic() {
                    self.get_identifier(ch)
                } else if ch.is_ascii_digit() {
                    self.get_numeric(ch)
                } else {
                    self.create_token(TokenType::Illegal)
                }
            }

            // EOF + Illegal
            
            None => self.create_token(TokenType::EOF)
        };
        token
    }

    fn get_peek_token(&mut self, peek: char, a:TokenType, b: TokenType) -> Token {
        if Some(peek) == self.peek_ch {
            self.next_char();
            self.create_token(b)
        } else {
            self.create_token(a)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_ch {
            if ch == '\n' { self.curr_line += 1; }
            if !ch.is_whitespace() { break; }
            self.next_char();
        }
    }

    fn get_identifier(&mut self, ch: char) -> Token {
        let mut identifier = String::new();
        identifier.push(ch);

        while let Some(c) = self.peek_ch {
            if c.is_ascii_alphanumeric() || c == '_' {
                identifier.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "define" => self.create_token(TokenType::Define),
            "fn" => self.create_token(TokenType::Function),

            "true" => self.create_token(TokenType::True),
            "false" => self.create_token(TokenType::False),
            
            "if" => self.create_token(TokenType::If),
            "else" => self.create_token(TokenType::Else),
            "return" => self.create_token(TokenType::Return),

            "use" => self.create_token(TokenType::Use),

            _ => self.create_token(TokenType::Identifier(identifier))
        }
    }

    fn get_numeric(&mut self, ch: char) -> Token {
        let mut is_float = false;
        let mut number = String::new();
        number.push(ch);

        while let Some(c) = self.peek_ch {
            if c.is_ascii_digit() {
                number.push(c);
                self.next_char();
            } else if c == '.' && !is_float {
                is_float = true;
                number.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        self.create_token(TokenType::Number(number))
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.curr_line)
    }
    
}

impl LexerTrait for Lexer {
    type Token = Token;

    fn next(&mut self) -> &Token {
        self.curr_tok = self.peek_tok.clone();
        self.peek_tok = self.next_token();
        &self.curr_tok
    }

    fn peek(&self) -> &Token {
        &self.peek_tok
    }

    fn curr(&self) -> &Token {
        &self.curr_tok
    }
}

// #[test]
// fn lexer() {
//     unimplemented!()
// }
