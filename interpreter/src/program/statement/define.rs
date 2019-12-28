use crate::TokenType;
use crate::Lexer;

use crate::parser::expect_peek;

use crate::Result;
use crate::error::ParseError;

use crate::traits::ParseTrait;
use crate::traits::LexerTrait;

use crate::program::expression::Expression;
use crate::program::expression::Identifier;

/// Define
/// 
/// mod Sine {
///   t: INPUT;
///   freq: INPUT;
///   phase: INPUT::Default(0);
///   vol: INPUT::Default(0);
/// 
///   OUTPUT [ self::vol*math::sin(2*PI*self::freq*self::t) ];
/// }
/// 
/// mod SineGen {
///   t: PARAM::Default(0);
///   f: PARAM::Default(440);
///   a: Sine { t: self::t; freq: self::f };
///   
///   RUN { self::t += 1; }
/// 
///   OUTPUT [ self::a::OUTPUT[0] ];
///   OUTPUT_CH_1(self::OUTPUT[0]);
///   OUTPUT_CH_2(self::OUTPUT[0]);
/// }
/// 
/// 
/// 
///
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Mod {
    name: Identifier,
    body: Vec<Expression>
}

impl Mod {
    pub fn new(name: Identifier, body: Vec<Expression>) -> Self {
        Self { name, body }
    }

    pub fn get_name(&self) -> &Identifier { &self.name }

    pub fn get_body(&self) -> &Vec<Expression> { &self.body }
}

impl ParseTrait for Mod {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        lexer.next();
        let name = Identifier::parse(lexer)?;
        let tok = lexer.peek().clone();
        expect_peek(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;

        let mut body = Vec::new();
        loop {
            let tok = lexer.next().token_type.clone();
            if TokenType::CloseCurlyBracket == tok { break; }
            body.push(Expression::parse(lexer)?)
        }

        Ok(Mod::new(name, body))
    }
}

// #[test]
// fn define_statement() {
//     unimplemented!()
// }


