use crate::Lexer;
use crate::TokenType;
use crate::Object;
use crate::Container;
use crate::Env;

use crate::parser::expect_peek;

use crate::traits::LexerTrait;
use crate::traits::ParseTrait;
use crate::traits::EvalTrait;

use crate::Result;
use crate::error::ParseError;

use crate::expression::Expression;
use crate::statement::Statement;

/// Cond Block
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct CondBlock {
    pub cond: Expression,
    pub cons: Vec<Statement>
}

impl CondBlock {
    pub fn new(cond: Expression, cons: Vec<Statement>) -> Self {
        Self {
            cond,
            cons
        }
    }
}

/// If 
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub cond_blocks: Vec<CondBlock>,
    pub otherwise: Option<Vec<Statement>>
}

impl If {
    pub fn new(
        cond_blocks: Vec<CondBlock>,
        otherwise: Option<Vec<Statement>>
    ) -> Self { 
        Self { 
            cond_blocks,
            otherwise
        } 
    }
}

impl ParseTrait for If {
    type Lexer = Lexer;

    fn parse(lexer: &mut Self::Lexer) -> Result<Self> {
        let mut cond_blocks = Vec::new();
        let mut otherwise = None;

        lexer.next();
        let exp = Expression::parse(lexer)?;
        let tok = lexer.peek().clone();
        expect_peek(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;
        let stmt = Statement::parse_block(lexer)?;
        cond_blocks.push(
            CondBlock::new(exp, stmt)
        );

        while lexer.peek().token_type == TokenType::Else {
            lexer.next();
            let tok = lexer.next();
            
            match tok.token_type {
                TokenType::If => {
                    lexer.next();
                    let exp = Expression::parse(lexer)?;
                    let tok = lexer.peek().clone();
                    expect_peek(lexer, TokenType::OpenCurlyBracket, ParseError::ExpectedOpenCurlyBracket(tok))?;
                    let stmt = Statement::parse_block(lexer)?;
                    cond_blocks.push(CondBlock::new(exp, stmt));
                },
                TokenType::OpenCurlyBracket => {
                    let stmt = Statement::parse_block(lexer)?;
                    otherwise = Some(stmt)
                }
                _ => return Err(ParseError::ExpectedSemicolon(tok.clone()))
            }
        }

        Ok(If::new(cond_blocks, otherwise))
    }
}

impl EvalTrait for If {
    type Object = Object;
    type Namespace = Env<Object>;

    fn eval(&self, env: Container<Self::Namespace>) -> Result<Object> {
        let mut result = Object::Null;
        for cond_block in self.cond_blocks.iter() {
            let cond = cond_block.cond.eval(env.clone())?;
            
            if cond.is_true() {
                for statement in cond_block.cons.iter() {
                    result = statement.eval(env.clone())?;
                }
                return Ok(result);
            }
        }

        if let Some(stmts) = &self.otherwise {
            for statement in stmts.iter() {
                result = statement.eval(env.clone())?;
            }
        } 

        Ok(result)
    }
}

#[cfg(test)]
mod if_tests {
    use crate::Object;
    use crate::Container;
    use crate::Env;
    use crate::Lexer;

    use crate::traits::LexerTrait;
    use crate::traits::ParseTrait;
    use crate::traits::EvalTrait;

    use crate::expression::Bool;
    use crate::expression::Real;
    use crate::expression::Expression;

    use crate::statement::Statement;

    use super::If;
    use super::CondBlock;

    #[test]
    fn if_exp() {
        let source = "if true { 1 }";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let if_exp = If::parse(&mut lexer);
        assert!(if_exp.is_ok(), "If parse failed: {:?}", if_exp);
        let if_exp = if_exp.unwrap();

        let ast = If::new(
            vec![
                CondBlock::new(
                    Expression::Bool(Bool::new(true)),
                    vec![
                        Statement::Expression(
                            Expression::Real(
                                Real::new(1.0)
                            )
                        )
                    ]
                )
            ],
            None
        );

        assert_eq!(if_exp, ast, "If ast failed");
        
        let obj = if_exp.eval(env);
        assert!(obj.is_ok(), "If eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(1.0));
    }

    #[test]
    fn if_exp_null() {
        let source = "if false { 1 }";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let if_exp = If::parse(&mut lexer);
        assert!(if_exp.is_ok(), "If parse failed: {:?}", if_exp);
        let if_exp = if_exp.unwrap();
        
        let ast = If::new(
            vec![
                CondBlock::new(
                    Expression::Bool(Bool::new(false)),
                    vec![
                        Statement::Expression(
                            Expression::Real(
                                Real::new(1.0)
                            )
                        )
                    ]
                )
            ],
            None
        );

        assert_eq!(if_exp, ast, "If ast failed");

        let obj = if_exp.eval(env);
        assert!(obj.is_ok(), "If eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Null);
    }

    #[test]
    fn if_else_exp() {
        let source = "if false { 1 } else { 2 }";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let if_exp = If::parse(&mut lexer);
        assert!(if_exp.is_ok(), "If parse failed: {:?}", if_exp);
        let if_exp = if_exp.unwrap();

        let ast = If::new(
            vec![
                CondBlock::new(
                    Expression::Bool(Bool::new(false)),
                    vec![
                        Statement::Expression(
                            Expression::Real(
                                Real::new(1.0)
                            )
                        )
                    ]
                )
            ],
            Some(vec![
                Statement::Expression(
                    Expression::Real(Real::new(2.0))
                )
            ])
        );

        assert_eq!(if_exp, ast, "If ast failed");
        
        let obj = if_exp.eval(env);
        assert!(obj.is_ok(), "If eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(2.0));
    }

    #[test]
    fn if_else_if_exp() {
        let source = "if false { 1 } else if true { 2 }";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let if_exp = If::parse(&mut lexer);
        assert!(if_exp.is_ok(), "If parse failed: {:?}", if_exp);
        let if_exp = if_exp.unwrap();

        let ast = If::new(
            vec![
                CondBlock::new(
                    Expression::Bool(Bool::new(false)),
                    vec![
                        Statement::Expression(
                            Expression::Real(
                                Real::new(1.0)
                            )
                        )
                    ]
                ),
                CondBlock::new(
                    Expression::Bool(Bool::new(true)),
                    vec![
                        Statement::Expression(
                            Expression::Real(
                                Real::new(2.0)
                            )
                        )
                    ]
                )
            ],
            None
        );

        assert_eq!(if_exp, ast, "If ast failed");
        
        let obj = if_exp.eval(env);
        assert!(obj.is_ok(), "If eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(2.0));
    }

    #[test]
    fn if_else_if_exp_null() {
        let source = "if false { 1 } else if false { 2 }";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let if_exp = If::parse(&mut lexer);
        assert!(if_exp.is_ok(), "If parse failed: {:?}", if_exp);
        let if_exp = if_exp.unwrap();

        let ast = If::new(
            vec![
                CondBlock::new(
                    Expression::Bool(Bool::new(false)),
                    vec![
                        Statement::Expression(
                            Expression::Real(
                                Real::new(1.0)
                            )
                        )
                    ]
                ),
                CondBlock::new(
                    Expression::Bool(Bool::new(false)),
                    vec![
                        Statement::Expression(
                            Expression::Real(
                                Real::new(2.0)
                            )
                        )
                    ]
                )
            ],
            None
        );

        assert_eq!(if_exp, ast, "If ast failed");
        
        let obj = if_exp.eval(env);
        assert!(obj.is_ok(), "If eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Null);
    }

    #[test]
    fn if_else_if_else_exp() {
        let source = "if false { 1 } else if false { 2 } else { 3 }";
        let env = Container::new(Env::new());

        let mut lexer = Lexer::new(String::from(source));
        lexer.next();

        let if_exp = If::parse(&mut lexer);
        assert!(if_exp.is_ok(), "If parse failed: {:?}", if_exp);
        let if_exp = if_exp.unwrap();

        let ast = If::new(
            vec![
                CondBlock::new(
                    Expression::Bool(Bool::new(false)),
                    vec![
                        Statement::Expression(
                            Expression::Real(
                                Real::new(1.0)
                            )
                        )
                    ]
                ),
                CondBlock::new(
                    Expression::Bool(Bool::new(false)),
                    vec![
                        Statement::Expression(
                            Expression::Real(
                                Real::new(2.0)
                            )
                        )
                    ]
                )
            ],
            Some(vec![
                Statement::Expression(
                    Expression::Real(Real::new(3.0))
                )
            ])
        );

        assert_eq!(if_exp, ast, "If ast failed");
        
        let obj = if_exp.eval(env);
        assert!(obj.is_ok(), "If eval failed: {:?}", obj);
        let obj = obj.unwrap();

        assert_eq!(obj, Object::Real(3.0));
    }
}