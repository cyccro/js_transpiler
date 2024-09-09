use crate::lexer::Token;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Identifier(String),
    StrLit(String),
    FnCall {
        fname: Box<Expr>,
        params: Box<Vec<Expr>>,
    },
    Accessor {
        parent: Box<Expr>,
        child: Box<Expr>,
    },
    Globals(Box<Vec<Expr>>),
}
pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
    len: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    content: String,
}
impl ParseError {
    pub fn new<T: ToString>(content: T) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            len: tokens.len(),
            tokens,
            idx: 0,
        }
    }
    fn peek_next(&self) -> Result<&Token, ParseError> {
        if let Some(tk) = self.tokens.get(self.idx + 1) {
            Ok(tk)
        } else {
            Err(ParseError {
                content: "Tried to get tokens but couldnt".to_string(),
            })
        }
    }
    fn peek(&self) -> Result<Token, ParseError> {
        if let Some(tk) = self.tokens.get(self.idx) {
            Ok(tk.clone())
        } else {
            Err(ParseError {
                content: "Tried to get tokens but couldnt".to_string(),
            })
        }
    }
    fn expect(&self, tk: &Token, msg: Option<String>) -> Result<Token, ParseError> {
        let token = self.peek()?;
        if token.cmp(tk) {
            Ok(token)
        } else {
            Err(ParseError {
                content: msg.unwrap_or(format!(
                    "Expected token with type {tk:?} but instead got {token:?}"
                )),
            })
        }
    }
    fn eat(&mut self) -> Result<Token, ParseError> {
        let tk = self.peek()?;
        self.idx += 1;
        Ok(tk)
    }
    pub fn create_program(&mut self) -> Result<Expr, ParseError> {
        let mut globals = Vec::new();
        while self.idx < self.len {
            globals.push(self.parse_expr()?);
            if Token::SemiColon.cmp(&self.peek()?) {
                let _ = self.eat();
            }
            if Token::Eof.cmp(&self.eat()?) {
                break;
            }
        }
        Ok(Expr::Globals(Box::new(globals)))
    }
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let accessor = self.parse_accessor()?;
        Ok(accessor)
    }
    fn parse_accessor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        match expr {
            Expr::Identifier(_) => {}
            _ => return Ok(expr),
        }
        while matches!(self.peek()?, Token::Dot) {
            let _ = self.eat();
            let child = self.parse_primary()?;
            match child {
                Expr::Identifier(_) => {}
                _ => return Err(ParseError::new("Expected a identifier")),
            };
            expr = Expr::Accessor {
                parent: Box::new(expr),
                child: Box::new(child),
            };
            if self.peek()?.cmp(&Token::LeftParen) {
                let _ = self.eat();
                expr = Expr::FnCall {
                    fname: Box::new(expr),
                    params: self.parse_params()?,
                };
                break;
            }
        }
        Ok(expr)
    }
    fn parse_params(&mut self) -> Result<Box<Vec<Expr>>, ParseError> {
        let mut exprs = Vec::new();
        while !self.peek()?.cmp(&Token::RightParen) {
            let expr = self.parse_expr()?;
            exprs.push(expr);
            if self.peek()?.cmp(&Token::RightParen) {
                let _ = self.eat();
                break;
            }
        }
        Ok(Box::new(exprs))
    }
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        Ok(match self.peek()? {
            Token::Identifier(str) => {
                let _ = self.eat();
                Expr::Identifier(str)
            }
            Token::StrLiteral(str) => {
                let _ = self.eat();
                Expr::StrLit(str)
            }
            Token::LeftParen => {
                let _ = self.eat();
                let expr = self.parse_expr()?;
                let _ = self.expect(&Token::RightParen, None);
                expr
            }
            tk => return Err(ParseError::new(format!("Invalid token {tk:?}"))),
        })
    }
}
