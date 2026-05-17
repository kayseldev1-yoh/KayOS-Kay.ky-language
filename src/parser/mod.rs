//! Kay.ky Parser - Generates AST from Tokens

use crate::lexer::Token;
use crate::ast::{Expr, Stmt, Literal, BinaryOp, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if self.peek() == Token::Newline {
                self.advance();
                continue;
            }
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Say => self.say_statement(),
            Token::Define => self.define_statement(),
            Token::If => self.if_statement(),
            Token::While => self.while_statement(),
            Token::Return => self.return_statement(),
            Token::Import => self.import_statement(),
            Token::Identifier(_) if self.peek_next() == Token::As => self.assign_statement(),
            _ => self.expression_statement(),
        }
    }

    fn say_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // say
        let expr = self.expression()?;
        self.consume_newline()?;
        Ok(Stmt::Say(expr))
    }

    fn define_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // define
        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => return Err("Expected function name after 'define'".into()),
        };

        let mut params = Vec::new();
        if self.match_token(Token::LParen) {
            if self.peek() != Token::RParen {
                loop {
                    match self.advance() {
                        Token::Identifier(param) => params.push(param),
                        _ => return Err("Expected parameter name".into()),
                    }
                    if !self.match_token(Token::Comma) { break; }
                }
            }
            self.consume(Token::RParen, "Expected ')' after parameters")?;
        }

        self.consume(Token::Colon, "Expected ':' before function body")?;
        let body = self.block()?;
        Ok(Stmt::Define(name, params, body))
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // if
        let condition = self.expression()?;
        self.consume(Token::Colon, "Expected ':' after if condition")?;
        let then_branch = self.block()?;
        
        let mut else_branch = None;
        if self.match_token(Token::Else) {
            if self.match_token(Token::Colon) {
                else_branch = Some(self.block()?);
            } else if self.peek() == Token::If {
                else_branch = Some(vec![self.if_statement()?]);
            } else {
                return Err("Expected ':' or 'if' after 'else'".into());
            }
        }
        
        Ok(Stmt::If(condition, then_branch, else_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // while
        let condition = self.expression()?;
        self.consume(Token::Colon, "Expected ':' after while condition")?;
        let body = self.block()?;
        Ok(Stmt::While(condition, body))
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // return
        let mut value = None;
        if self.peek() != Token::Newline && self.peek() != Token::Eof {
            value = Some(self.expression()?);
        }
        self.consume_newline()?;
        Ok(Stmt::Return(value))
    }

    fn import_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // import
        let name = match self.advance() {
            Token::Identifier(name) => name,
            Token::String(name) => name,
            _ => return Err("Expected module name after 'import'".into()),
        };
        self.consume_newline()?;
        Ok(Stmt::Import(name))
    }

    fn assign_statement(&mut self) -> Result<Stmt, String> {
        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };
        self.advance(); // as
        let expr = self.expression()?;
        self.consume_newline()?;
        Ok(Stmt::Assign(name, expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume_newline()?;
        Ok(Stmt::Expression(expr))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        self.consume_newline()?;
        self.consume(Token::Indent, "Expected indentation for block")?;
        let mut statements = Vec::new();
        while !self.is_at_end() && self.peek() != Token::Dedent {
            if self.peek() == Token::Newline {
                self.advance();
                continue;
            }
            statements.push(self.statement()?);
        }
        self.consume(Token::Dedent, "Expected dedent after block")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_any(&[Token::EqualEqual, Token::BangEqual]) {
            let operator = match self.previous() {
                Token::EqualEqual => BinaryOp::Equal,
                Token::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_any(&[Token::Greater, Token::Less]) {
            let operator = match self.previous() {
                Token::Greater => BinaryOp::Greater,
                Token::Less => BinaryOp::Less,
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_any(&[Token::Plus, Token::Minus]) {
            let operator = match self.previous() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_any(&[Token::Star, Token::Slash, Token::FloorDiv, Token::Modulo]) {
            let operator = match self.previous() {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::FloorDiv => BinaryOp::FloorDiv,
                Token::Modulo => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_any(&[Token::Minus]) {
            let operator = UnaryOp::Negate;
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;
        loop {
            if self.match_token(Token::LParen) {
                let mut arguments = Vec::new();
                if self.peek() != Token::RParen {
                    loop {
                        arguments.push(self.expression()?);
                        if !self.match_token(Token::Comma) { break; }
                    }
                }
                self.consume(Token::RParen, "Expected ')' after arguments")?;
                expr = Expr::Call(Box::new(expr), arguments);
            } else if self.match_token(Token::Dot) {
                match self.advance() {
                    Token::Identifier(name) => {
                        expr = Expr::Get(Box::new(expr), name);
                    }
                    _ => return Err("Expected property name after '.'".into()),
                }
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Boolean(b) => Ok(Expr::Literal(Literal::Boolean(b))),
            Token::Null => Ok(Expr::Literal(Literal::Null)),
            Token::Number(n) => Ok(Expr::Literal(Literal::Number(n))),
            Token::String(s) => Ok(Expr::Literal(Literal::String(s))),
            Token::Identifier(name) => Ok(Expr::Variable(name)),
            Token::LParen => {
                let expr = self.expression()?;
                self.consume(Token::RParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            _ => Err(format!("Expected expression, found {:?}", self.previous())),
        }
    }

    fn match_token(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_any(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: Token) -> bool {
        if self.is_at_end() { return false; }
        self.peek() == token
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek() == Token::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).cloned().unwrap_or(Token::Eof)
    }

    fn peek_next(&self) -> Token {
        self.tokens.get(self.current + 1).cloned().unwrap_or(Token::Eof)
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).cloned().unwrap_or(Token::Eof)
    }

    fn consume(&mut self, token: Token, message: &str) -> Result<Token, String> {
        if self.check(token) { return Ok(self.advance()); }
        Err(message.into())
    }

    fn consume_newline(&mut self) -> Result<(), String> {
        if self.match_token(Token::Newline) || self.is_at_end() || self.peek() == Token::Dedent {
            Ok(())
        } else {
            Err(format!("Expected newline or end of statement, found {:?}", self.peek()))
        }
    }
}
