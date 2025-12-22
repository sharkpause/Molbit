use crate::token::Token

enum Statement {
    Return(Expression)
}

enum Expression {
    IntLiteral(i64)
}

impl Statement {
    fn parse_program(&self, tokens: Vec<Token>) -> Statement {

    }
    
    fn parse_statement(&self, tokens: Vec<Token>) -> Statement {

    }

    fn parse_expression(&self, tokens: Vec<Token>) -> Expression {

    }
}