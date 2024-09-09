use std::cmp::Eq;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Dot,
    Identifier(String),
    StrLiteral(String),
    SemiColon,
    LeftParen,
    RightParen,
    Eof,
}

#[derive(Debug)]
pub struct Cursor {
    line: usize,
    column: usize,
    offset: usize,
}

fn is_identifier_initializer(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}
fn is_identifier_char(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

impl Token {
    pub fn cmp(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Token::Dot, Token::Dot) => true,
            (Token::LeftParen, Token::LeftParen) => true,
            (Token::RightParen, Token::RightParen) => true,
            (Token::SemiColon, Token::SemiColon) => true,
            (Token::Eof, Token::Eof) => true,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (Token::StrLiteral(_), Token::StrLiteral(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    text: String,
    cursor: Cursor,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            offset: 0,
            line: 0,
            column: 0,
        }
    }
}

impl Lexer {
    pub fn new<T: ToString>(content: T) -> Self {
        Self {
            text: content.to_string(),
            cursor: Cursor::new(),
        }
    }
    fn get_str_literal(chars: &Vec<char>, mut idx: usize) -> (Token, usize) {
        let mut lit = String::new();
        while let Some(c) = chars.get(idx) {
            if *c == '"' {
                break;
            }
            lit.push(*c);
            idx += 1;
        }
        (Token::StrLiteral(lit), idx)
    }
    fn get_identifier(chars: &Vec<char>, mut idx: usize) -> (Token, usize) {
        let mut txt = String::new();
        while let Some(char) = chars.get(idx) {
            if is_identifier_char(*char) {
                txt.push(*char);
                idx += 1;
            } else {
                break;
            }
        }
        (Token::Identifier(txt), idx)
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(self.text.len());
        let lines = self.text.lines();
        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            while let Some(c) = chars.get(self.cursor.column) {
                let token = match c {
                    ' ' => {
                        self.cursor.column += 1;
                        continue;
                    }
                    '.' => Token::Dot,
                    '(' => Token::LeftParen,
                    ')' => Token::RightParen,
                    ';' => Token::SemiColon,
                    '"' => {
                        let (literal, n) = Self::get_str_literal(&chars, self.cursor.column + 1);
                        self.cursor.column = n;
                        literal
                    }
                    ch if is_identifier_initializer(*ch) => {
                        let (identifier, n) = Self::get_identifier(&chars, self.cursor.column);
                        self.cursor.column = n - 1;
                        identifier
                    }
                    _ => {
                        self.cursor.column += 1;
                        continue;
                    }
                };
                tokens.push(token);
                self.cursor.column += 1;
            }
            self.cursor.line += 1;
            self.cursor.offset += 1;
            self.cursor.column = 0;
        }
        tokens.push(Token::Eof);
        tokens
    }
}
