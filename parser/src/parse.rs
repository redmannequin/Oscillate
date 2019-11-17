use crate::Lexer;
use crate::Token;
use crate::TokenType;
use crate::ast::{Program, Statment, Expression};

use crate::Result;
use crate::error::ParseError;

pub struct Parser {
    lexer: Lexer,
    errors: Vec<ParseError>,

    curr_token: Token,
    peek_token: Token,
}

impl Parser {

    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            errors: Vec::new(),
            curr_token: Token::new(TokenType::Illegal, 0),
            peek_token: Token::new(TokenType::Illegal, 0),
        };
        parser.next_token();
        parser
    }

    fn next_token(&mut self) -> Token {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        self.curr_token.clone()
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut program = Program::new();

        while self.peek_token.token_type != TokenType::EOF {
            match self.parse_statement() {
                Ok(stmt) => program.add_statment(stmt),
                Err(err) => self.errors.push(err)
            }
        }
        
        Ok(program)
    }

    /// 
    /// Statements
    /// 

    fn parse_statement(&mut self) -> Result<Statment> {
        match self.peek_token.token_type {
            TokenType::Let => Ok(self.parse_let_statement()?),
            _ => Ok(self.parse_expression_statement()?)
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statment> {
        let _let = self.next_token();
        let ident_token = self.next_token();
        let name = match ident_token.token_type {
            TokenType::Identifier(ident) => { ident },
            _ => return Err(ParseError::ExpectedIdentifier) 
        };

        self.expect_peek(TokenType::Assing, ParseError::InvalidSoruce)?;

        let expr = self.parse_expression()?;
        self.expect_semicolon()?;
        Ok(Statment::Let(name, expr))
    }


    fn parse_expression_statement(&mut self) -> Result<Statment> {
        let expression = self.parse_expression()?;
        self.expect_semicolon()?;
        Ok(Statment::Expression(expression))
    }

    ///
    /// Expressions
    /// 

    fn parse_expression(&mut self) -> Result<Expression> {
        let token = self.next_token();
        match token.token_type {
            TokenType::Identifier(_) => self.parse_identifier(),
            TokenType::Start => Ok(Expression::Start),
            TokenType::Stop => Ok(Expression::Stop),
            TokenType::Exit => Ok(Expression::Exit),
            _ => Err(ParseError::ExpectedExpression),
        }
    }

    fn parse_identifier(&mut self) -> Result<Expression> {
        let name = match &self.curr_token.token_type {
            TokenType::Identifier(ident) => ident.clone(),
            _ => return Err(ParseError::ExpectedIdentifier) 
        };
        Ok(Expression::Identifier(name))
    }

    ///
    /// Helpers
    /// 
    
    fn expect_peek(&mut self, token_type: TokenType, error: ParseError) -> Result<()> {
        let token = self.next_token();
        if token.token_type == token_type {
            Ok(())
        } else {
            Err(error)
        }
    }

    fn expect_semicolon(&mut self) -> Result<()> {
        self.expect_peek(TokenType::Semicolon, ParseError::ExpectedSemicolon)
    }

    pub fn get_errors(&self) -> &Vec<ParseError> {
        &self.errors
    }

    pub fn clear_errors(&mut self) -> Vec<ParseError> {
        self.errors.drain(..).collect()
    }
    
}