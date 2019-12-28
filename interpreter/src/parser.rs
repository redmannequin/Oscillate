use crate::Lexer;
use crate::TokenType;

use crate::traits::LexerTrait;

use crate::Result;
use crate::error::ParseError;

pub fn expect_curr(lexer: &mut Lexer, tok_type: TokenType, err: ParseError) -> Result<()> {
    let token = lexer.curr();
    if token.token_type == tok_type {
        Ok(())
    } else {
        Err(err)
    }
}

pub fn expect_peek(lexer: &mut Lexer, tok_type: TokenType, err: ParseError) -> Result<()> {
    let token = lexer.next();
    if token.token_type == tok_type {
        Ok(())
    } else {
        Err(err)
    }
}

pub fn on_semicolon(lexer: &mut Lexer) -> Result<()> {
    let tok = lexer.curr();
    if tok.token_type == TokenType::Semicolon {
        Ok(())
    } else {
        Err(ParseError::ExpectedSemicolon(tok.clone()))
    }
}

// #[test]
// fn parser() {
//     unimplemented!()
// }
