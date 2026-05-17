//! Kay.ky Lexer - Tokenizes Kay.ky source code
//! 
//! Syntax design: Indentation-based, minimalist, futuristic.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Identifier(String),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    
    // Keywords
    Define, // define
    Say,    // say
    As,     // as
    If,     // if
    Else,   // else
    While,  // while
    For,    // for
    In,     // in
    Return, // return
    Import, // import
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    FloorDiv,
    Modulo,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    Greater,
    
    // Symbols
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Dot,
    
    // Special
    Newline,
    Indent,
    Dedent,
    Eof,
}

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    indent_stack: Vec<usize>,
    pending_tokens: Vec<Token>,
    keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("define".into(), Token::Define);
        keywords.insert("say".into(), Token::Say);
        keywords.insert("as".into(), Token::As);
        keywords.insert("if".into(), Token::If);
        keywords.insert("else".into(), Token::Else);
        keywords.insert("while".into(), Token::While);
        keywords.insert("for".into(), Token::For);
        keywords.insert("in".into(), Token::In);
        keywords.insert("return".into(), Token::Return);
        keywords.insert("import".into(), Token::Import);
        keywords.insert("true".into(), Token::Boolean(true));
        keywords.insert("false".into(), Token::Boolean(false));
        keywords.insert("null".into(), Token::Null);
        
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            indent_stack: vec![0],
            pending_tokens: Vec::new(),
            keywords,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            if !self.pending_tokens.is_empty() {
                tokens.push(self.pending_tokens.remove(0));
                continue;
            }

            let start_of_line = self.column == 1;
            
            if start_of_line {
                self.handle_indentation()?;
            }

            if self.is_at_end() { break; }

            self.skip_whitespace();
            if self.is_at_end() { break; }

            let c = self.peek();
            
            if c == '\n' {
                tokens.push(Token::Newline);
                self.advance();
                self.line += 1;
                self.column = 1;
                continue;
            }

            let token = match c {
                '#' => { self.skip_comment(); continue; }
                '"' => self.read_string()?,
                '0'..='9' => self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
                '(' => { self.advance(); Token::LParen }
                ')' => { self.advance(); Token::RParen }
                '[' => { self.advance(); Token::LBracket }
                ']' => { self.advance(); Token::RBracket }
                ',' => { self.advance(); Token::Comma }
                ':' => { self.advance(); Token::Colon }
                '.' => { self.advance(); Token::Dot }
                '+' => { self.advance(); Token::Plus }
                '-' => { self.advance(); Token::Minus }
                '*' => { self.advance(); Token::Star }
                '/' => {
                    self.advance();
                    if self.peek() == '/' {
                        self.advance();
                        Token::FloorDiv
                    } else {
                        Token::Slash
                    }
                }
                '%' => { self.advance(); Token::Modulo }
                '=' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        Token::EqualEqual
                    } else {
                        Token::Equal
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        Token::BangEqual
                    } else {
                        return Err(format!("Unexpected character '!' at {}:{}", self.line, self.column));
                    }
                }
                '<' => { self.advance(); Token::Less }
                '>' => { self.advance(); Token::Greater }
                _ => return Err(format!("Unexpected character '{}' at {}:{}", c, self.line, self.column)),
            };
            tokens.push(token);
        }
        
        // Final dedents
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            tokens.push(Token::Dedent);
        }
        
        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn handle_indentation(&mut self) -> Result<(), String> {
        let mut indent = 0;
        while !self.is_at_end() && (self.peek() == ' ' || self.peek() == '\t') {
            if self.peek() == ' ' {
                indent += 1;
            } else {
                indent += 4; // Tab is 4 spaces
            }
            self.advance();
        }

        if self.is_at_end() || self.peek() == '\n' || self.peek() == '#' {
            return Ok(());
        }

        let current_indent = *self.indent_stack.last().unwrap();
        if indent > current_indent {
            self.indent_stack.push(indent);
            self.pending_tokens.push(Token::Indent);
        } else if indent < current_indent {
            while indent < *self.indent_stack.last().unwrap() {
                self.indent_stack.pop();
                self.pending_tokens.push(Token::Dedent);
            }
            if indent != *self.indent_stack.last().unwrap() {
                return Err(format!("Inconsistent indentation at {}:{}", self.line, self.column));
            }
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.advance(); // "
        let mut s = String::new();
        while !self.is_at_end() && self.peek() != '"' {
            s.push(self.advance());
        }
        if self.is_at_end() {
            return Err(format!("Unterminated string at {}:{}", self.line, self.column));
        }
        self.advance(); // "
        Ok(Token::String(s))
    }

    fn read_number(&mut self) -> Token {
        let mut s = String::new();
        while !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek() == '.') {
            s.push(self.advance());
        }
        Token::Number(s.parse().unwrap_or(0.0))
    }

    fn read_identifier(&mut self) -> Token {
        let mut s = String::new();
        while !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == '_') {
            s.push(self.advance());
        }
        if let Some(token) = self.keywords.get(&s) {
            token.clone()
        } else {
            Token::Identifier(s)
        }
    }

    fn peek(&self) -> char {
        self.source.get(self.position).copied().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.position += 1;
        self.column += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }
}
