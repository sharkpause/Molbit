use crate::token::{ Token, TokenKind };

#[derive(Debug)]
pub enum LexerError {
    GenericError,
    UnexpectedChar {
        character: char,
        line: usize,
        column: usize
    },
    EndOfInput,
    UnknownToken {
        character: char,
        line: usize,
        column: usize
    }
}

pub struct Lexer {
    pub input: Vec<char>,
    pub index: usize,
    pub line: usize,
    pub column: usize
}

impl Lexer {
    pub fn from(input: String) -> Self {
        return Self {
            input: input.chars().collect(),
            index: 0,
            line: 1,
            column: 1
        };
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        return self.input.get(self.index + offset).copied();
    }

    fn consume_char(&mut self) -> Option<char> {
        let character = self.peek_char(0)?;
        self.index += 1;

        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        return Some(character);
    }

    fn skip_whitespace(&mut self) {
        while let Some(character) = self.peek_char(0) {
            if character.is_whitespace() {
                self.consume_char();
            } else {
                break;
            }
        }
    }

    fn single_char_token(&mut self) -> Option<Token> {
        let character = self.peek_char(0)?;
        let line = self.line;
        let column = self.column;

        let kind = match character {
            '(' => TokenKind::LeftParentheses,
            ')' => TokenKind::RightParentheses,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ';' => TokenKind::Semicolon,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '=' => TokenKind::Equal,
            ',' => TokenKind::Comma,
            '<' => TokenKind::LessThan,
            '>' => TokenKind::GreaterThan,
            '!' => TokenKind::Not,
            _ => return None,
        };

        self.consume_char();

        return Some(Token {
            kind,
            line,
            column,
        });
    }

    fn double_char_token(&mut self) -> Option<Token> {
        let line = self.line;
        let column = self.column;

        match self.peek_char(0)? {
            '=' => {
                if self.peek_char(1)? == '=' {
                    self.consume_char();
                    self.consume_char();

                    return Some(Token {
                        kind: TokenKind::DoubleEqual,
                        line,
                        column,
                    });
                } else {
                    return None;
                }
            },
            '!' => {
                if self.peek_char(1)? == '=' {
                    self.consume_char();
                    self.consume_char();

                    return Some(Token {
                        kind: TokenKind::NotEqual,
                        line,
                        column,
                    });
                } else {
                    return Some(Token {
                        kind: TokenKind::Not,
                        line,
                        column,
                    });
                }
            },
            '<' => {
                if self.peek_char(1)? == '=' {
                    self.consume_char();
                    self.consume_char();

                    return Some(Token {
                        kind: TokenKind::LessEqual,
                        line,
                        column,
                    });
                } else {
                    return None;
                }
            },
            '>' => {
                if self.peek_char(1)? == '=' {
                    self.consume_char();
                    self.consume_char();

                    return Some(Token {
                        kind: TokenKind::GreaterEqual,
                        line,
                        column,
                    });
                } else {
                    return None;
                }
            },
            '&' => {
                if self.peek_char(1)? == '&' {
                    self.consume_char();
                    self.consume_char();

                    return Some(Token {
                        kind: TokenKind::And,
                        line,
                        column,
                    });
                } else {
                    return None;
                }
            },
            '|' => {
                if self.peek_char(1)? == '|' {
                    self.consume_char();
                    self.consume_char();

                    return Some(Token {
                        kind: TokenKind::Or,
                        line,
                        column,
                    });
                } else {
                    return None;
                }
            },
            _ => None,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        let current_char =
            self.peek_char(0).ok_or(LexerError::EndOfInput)?;

        let start_line = self.line;
        let start_column = self.column;
        let mut token = String::new();

        if current_char.is_digit(10) {
            while let Some(character) = self.peek_char(0) {
                if character.is_digit(10) {
                    token.push(character);
                    self.consume_char();
                } else {
                    if character.is_alphabetic() {
                        return Err(LexerError::UnexpectedChar{
                            character, line: start_line, column: start_column
                        });
                    }
                    break;
                }
            }

            let number: i64 =
                token.parse().map_err(
                |_| LexerError::UnexpectedChar {
                    character: token.chars().last().unwrap_or(' '),
                    line: start_line, column: start_column
                })?;
        
            return Ok( Token {
                kind: TokenKind::IntLiteral(number),
                line: start_line,
                column: start_column
            });
        } else if current_char.is_alphanumeric() {
            if let Some(first) = self.peek_char(0) {
                if first.is_alphabetic() || first == '_' {
                    token.push(first);
                    self.consume_char();
                
                    while let Some(character) = self.peek_char(0) {
                        if character.is_alphanumeric() || character == '_' {
                            token.push(character);
                            self.consume_char();
                        } else {
                            break;
                        }
                    }
                } else {
                    return Err(LexerError::UnexpectedChar { character: first, line: start_line, column: start_column });
                }
            }

            let kind = match token.as_str() {
                "return" => Ok(TokenKind::Return),
                "function" => Ok(TokenKind::Function),
                "int" => Ok(TokenKind::IntType),
                "var" => Ok(TokenKind::Var),
                "if" => Ok(TokenKind::If),
                "else" => Ok(TokenKind::Else),
                "while" => Ok(TokenKind::While),
                "break" => Ok(TokenKind::Break),
                "continue" => Ok(TokenKind::Continue),
                _ => Ok(TokenKind::Identifier(token)),
            }?;

            return Ok( Token {
                kind,
                line: start_line,
                column: start_column
            });
        } else {
            if let Some(token) = self.double_char_token() {
                return Ok(token);
            } else if let Some(token) = self.single_char_token() {
                return Ok(token);
            }

            return Err(LexerError::UnknownToken{ character: current_char, line: start_line, column: start_column });
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Ok(token) => {
                    tokens.push(token);
                },
                Err(LexerError::EndOfInput) => {
                    break;
                },
                Err(error) => {
                    return Err(error);
                }
            }
        }

        return Ok(tokens);
    }
}