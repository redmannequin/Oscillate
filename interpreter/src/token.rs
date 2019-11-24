use crate::traits::TokenTrait;

/// TokenType
/// 
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    EOF,

    // Identifier + lLterals 
    Identifier(String),
    Number(String),
    String(String),

    // Operators
    Assing,
    
    Plus,
    Minus,
    Divide,

    Star,
    DoubleStar,
    
    Not,

    Equal,
    NotEqual,

    GraterThan,
    LessThan,

    Dot,
    
    // Delimiters
    Comma,
    Colon,
    DoubleColon,
    Semicolon,

    OpenRoundBracket,
    CloseRoundBracket,

    OpenCurlyBracket,
    CloseCurlyBracket,

    OpenSquareBracket,
    CloseSquareBracket,

    // Keywords
    Function,
    Define,

    True,
    False,
    If,
    Else,
    Return,

    Use,
    Set
}

/// Token
/// 
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, line: u32) -> Self {
        Self {
            token_type: token_type,
            line: line
        }
    }
}

impl TokenTrait for Token {
    type TokenType = TokenType;

    fn get_type(&self) -> &Self::TokenType {
        &self.token_type
    }
    
    fn get_line(&self) -> u32 {
        self.line
    }
}