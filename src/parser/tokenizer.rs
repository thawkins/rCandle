//! G-Code Tokenizer/Lexer
//!
//! This module provides functionality to tokenize G-Code text into structured tokens.
//! It handles various G-Code formats including:
//! - Commands (G, M, T, etc.)
//! - Parameters (X, Y, Z, F, S, etc.)
//! - Comments (parentheses and semicolon styles)
//! - Line numbers (N)
//! - Checksums (*)

use crate::utils::error::{Error, Result};

/// Represents a single token in G-Code
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// G command (e.g., G0, G1, G2)
    GCommand(u32),
    /// M command (e.g., M3, M5)
    MCommand(u32),
    /// T command (tool change)
    TCommand(u32),
    /// S command (spindle speed)
    SCommand(f64),
    /// F command (feed rate)
    FCommand(f64),
    /// Parameter with letter and value (e.g., X10.5, Y-20.0)
    Parameter { letter: char, value: f64 },
    /// Line number (N)
    LineNumber(u32),
    /// Comment text (without delimiters)
    Comment(String),
    /// Checksum value
    Checksum(u32),
    /// End of line
    EndOfLine,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::GCommand(n) => write!(f, "G{}", n),
            Token::MCommand(n) => write!(f, "M{}", n),
            Token::TCommand(n) => write!(f, "T{}", n),
            Token::SCommand(v) => write!(f, "S{}", v),
            Token::FCommand(v) => write!(f, "F{}", v),
            Token::Parameter { letter, value } => write!(f, "{}{}", letter, value),
            Token::LineNumber(n) => write!(f, "N{}", n),
            Token::Comment(s) => write!(f, "({})", s),
            Token::Checksum(n) => write!(f, "*{}", n),
            Token::EndOfLine => write!(f, "\\n"),
        }
    }
}

/// G-Code tokenizer
pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    line: usize,
}

impl Tokenizer {
    /// Create a new tokenizer from input text
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
        }
    }

    /// Tokenize the entire input into a vector of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    /// Get the next token from the input
    fn next_token(&mut self) -> Result<Token> {
        let ch = self.peek();

        match ch {
            '\n' | '\r' => {
                self.advance();
                if ch == '\r' && self.peek() == '\n' {
                    self.advance();
                }
                self.line += 1;
                Ok(Token::EndOfLine)
            }
            ';' => {
                // Semicolon comment - rest of line
                self.advance();
                let comment = self.read_until_newline();
                Ok(Token::Comment(comment))
            }
            '(' => {
                // Parenthesis comment
                self.advance();
                let comment = self.read_until(')');
                if self.peek() == ')' {
                    self.advance();
                }
                Ok(Token::Comment(comment))
            }
            '*' => {
                // Checksum
                self.advance();
                let value = self.read_number()?;
                Ok(Token::Checksum(value as u32))
            }
            'G' | 'g' => {
                self.advance();
                let value = self.read_number()?;
                Ok(Token::GCommand(value as u32))
            }
            'M' | 'm' => {
                self.advance();
                let value = self.read_number()?;
                Ok(Token::MCommand(value as u32))
            }
            'T' | 't' => {
                self.advance();
                let value = self.read_number()?;
                Ok(Token::TCommand(value as u32))
            }
            'S' | 's' => {
                self.advance();
                let value = self.read_number()?;
                Ok(Token::SCommand(value))
            }
            'F' | 'f' => {
                self.advance();
                let value = self.read_number()?;
                Ok(Token::FCommand(value))
            }
            'N' | 'n' => {
                self.advance();
                let value = self.read_number()?;
                Ok(Token::LineNumber(value as u32))
            }
            c if c.is_ascii_alphabetic() => {
                // Other parameters (X, Y, Z, I, J, K, R, P, etc.)
                let letter = c.to_ascii_uppercase();
                self.advance();
                let value = self.read_number()?;
                Ok(Token::Parameter { letter, value })
            }
            _ => Err(Error::Parse(format!(
                "Unexpected character '{}' at line {}",
                ch, self.line
            ))),
        }
    }

    /// Read a number (integer or float) from the input
    fn read_number(&mut self) -> Result<f64> {
        let start = self.position;
        let mut has_dot = false;
        let mut has_digits = false;

        // Handle optional sign
        if self.peek() == '+' || self.peek() == '-' {
            self.advance();
        }

        // Read digits and optional decimal point
        while !self.is_at_end() {
            let ch = self.peek();
            if ch.is_ascii_digit() {
                has_digits = true;
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        if !has_digits {
            return Err(Error::Parse(format!(
                "Expected number at line {}",
                self.line
            )));
        }

        let number_str: String = self.input[start..self.position].iter().collect();
        number_str.parse::<f64>().map_err(|e| {
            Error::Parse(format!("Failed to parse number '{}': {}", number_str, e))
        })
    }

    /// Read until a specific character is found
    fn read_until(&mut self, delimiter: char) -> String {
        let start = self.position;
        while !self.is_at_end() && self.peek() != delimiter {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        self.input[start..self.position].iter().collect()
    }

    /// Read until newline
    fn read_until_newline(&mut self) -> String {
        let start = self.position;
        while !self.is_at_end() && self.peek() != '\n' && self.peek() != '\r' {
            self.advance();
        }
        self.input[start..self.position].iter().collect()
    }

    /// Skip whitespace characters (but not newlines)
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let ch = self.peek();
            if ch == ' ' || ch == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Get the current character without advancing
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    /// Advance to the next character
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
        }
    }

    /// Check if we've reached the end of input
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_gcode() {
        let input = "G0 X10 Y20 Z5";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::GCommand(0));
        assert_eq!(
            tokens[1],
            Token::Parameter {
                letter: 'X',
                value: 10.0
            }
        );
        assert_eq!(
            tokens[2],
            Token::Parameter {
                letter: 'Y',
                value: 20.0
            }
        );
        assert_eq!(
            tokens[3],
            Token::Parameter {
                letter: 'Z',
                value: 5.0
            }
        );
    }

    #[test]
    fn test_with_feed_rate() {
        let input = "G1 X100 F1000";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::GCommand(1));
        assert_eq!(
            tokens[1],
            Token::Parameter {
                letter: 'X',
                value: 100.0
            }
        );
        assert_eq!(tokens[2], Token::FCommand(1000.0));
    }

    #[test]
    fn test_spindle_command() {
        let input = "M3 S10000";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::MCommand(3));
        assert_eq!(tokens[1], Token::SCommand(10000.0));
    }

    #[test]
    fn test_parenthesis_comment() {
        let input = "G0 (This is a comment) X10";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::GCommand(0));
        assert_eq!(tokens[1], Token::Comment("This is a comment".to_string()));
        assert_eq!(
            tokens[2],
            Token::Parameter {
                letter: 'X',
                value: 10.0
            }
        );
    }

    #[test]
    fn test_semicolon_comment() {
        let input = "G0 X10 ; This is a comment";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::GCommand(0));
        assert_eq!(
            tokens[1],
            Token::Parameter {
                letter: 'X',
                value: 10.0
            }
        );
        assert_eq!(tokens[2], Token::Comment(" This is a comment".to_string()));
    }

    #[test]
    fn test_negative_coordinates() {
        let input = "G1 X-10.5 Y-20.75";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::GCommand(1));
        assert_eq!(
            tokens[1],
            Token::Parameter {
                letter: 'X',
                value: -10.5
            }
        );
        assert_eq!(
            tokens[2],
            Token::Parameter {
                letter: 'Y',
                value: -20.75
            }
        );
    }

    #[test]
    fn test_line_number() {
        let input = "N10 G0 X0";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::LineNumber(10));
        assert_eq!(tokens[1], Token::GCommand(0));
        assert_eq!(
            tokens[2],
            Token::Parameter {
                letter: 'X',
                value: 0.0
            }
        );
    }

    #[test]
    fn test_multiline() {
        let input = "G0 X0\nG1 Y10\nG2 Z5";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 8); // 3 commands + 3 parameters + 2 newlines (no trailing newline)
    }

    #[test]
    fn test_case_insensitive() {
        let input = "g0 x10 y20";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::GCommand(0));
        assert_eq!(
            tokens[1],
            Token::Parameter {
                letter: 'X',
                value: 10.0
            }
        );
    }

    #[test]
    fn test_arc_parameters() {
        let input = "G2 X10 Y10 I5 J5";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::GCommand(2));
        assert_eq!(
            tokens[3],
            Token::Parameter {
                letter: 'I',
                value: 5.0
            }
        );
        assert_eq!(
            tokens[4],
            Token::Parameter {
                letter: 'J',
                value: 5.0
            }
        );
    }
}
