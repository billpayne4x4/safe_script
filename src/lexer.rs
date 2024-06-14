use crate::types::var_type::VarType;
use crate::types::tokens::Tokens;
use crate::types::numeric::Numeric;
use crate::types::base_functions::BaseFunctions;

pub struct Lexer {
    input: String,
    pos: usize,
}

impl Lexer {
    pub fn new(script: &str) -> Self {
        Lexer {
            input: script.to_string(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Tokens> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            self.skip_whitespace();
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Tokens> {
        let input_slice = &self.input[self.pos..];
        let ch = input_slice.chars().next()?;

        match ch {
            ':' => { self.pos += 1; Some(Tokens::Colon) },
            ';' => { self.pos += 1; Some(Tokens::SemiColon) },
            '.' => { self.pos += 1; Some(Tokens::Period) },
            '=' => {
                match input_slice.get(0..2) {
                    Some("==") => { self.pos += 2; Some(Tokens::EqualEqual) },
                    Some("=+") => { self.pos += 2; Some(Tokens::EqualPlus) },
                    Some("=-") => { self.pos += 2; Some(Tokens::EqualMinus) },
                    Some("=*") => { self.pos += 2; Some(Tokens::EqualStar) },
                    Some("=/") => { self.pos += 2; Some(Tokens::EqualForwardSlash) },
                    _ => { self.pos += 1; Some(Tokens::Equals) }
                }
            },
            '+' => {
                match input_slice.get(0..2) {
                    Some("+=") => { self.pos += 2; Some(Tokens::PlusEqual) },
                    Some("++") => { self.pos += 2; Some(Tokens::PlusPlus) },
                    _ => { self.pos += 1; Some(Tokens::Plus) }
                }
            },
            '-' => {
                match input_slice.get(0..2) {
                    Some("-=") => { self.pos += 2; Some(Tokens::MinusEqual) },
                    Some("--") => { self.pos += 2; Some(Tokens::MinusMinus) },
                    _ => { self.pos += 1; Some(Tokens::Minus) }
                }
            },
            '*' => {
                match input_slice.get(0..2) {
                    Some("*=") => { self.pos += 2; Some(Tokens::EqualStar) },
                    _ => { self.pos += 1; Some(Tokens::Star) }
                }
            },
            '/' => {
                match input_slice.get(0..2) {
                    Some("/=") => { self.pos += 2; Some(Tokens::EqualForwardSlash) },
                    _ => { self.pos += 1; Some(Tokens::ForwardSlash) }
                }
            },
            '%' => { self.pos += 1; Some(Tokens::Percent) },
            '\\' => { self.pos += 1; Some(Tokens::BackSlash) },
            '!' => {
                match input_slice.get(0..2) {
                    Some("!=") => { self.pos += 2; Some(Tokens::NotEqual) },
                    _ => { self.pos += 1; Some(Tokens::Bang) }
                }
            },
            '>' => {
                match input_slice.get(0..2) {
                    Some(">=") => { self.pos += 2; Some(Tokens::GreaterEqual) },
                    _ => { self.pos += 1; Some(Tokens::Greater) }
                }
            },
            '<' => {
                match input_slice.get(0..2) {
                    Some("<=") => { self.pos += 2; Some(Tokens::LessEqual) },
                    _ => { self.pos += 1; Some(Tokens::Less) }
                }
            },
            '&' => {
                match input_slice.get(0..2) {
                    Some("&&") => { self.pos += 2; Some(Tokens::AndAnd) },
                    _ => { self.pos += 1; Some(Tokens::Ampersand) }
                }
            },
            '|' => {
                match input_slice.get(0..2) {
                    Some("||") => { self.pos += 2; Some(Tokens::Or) },
                    _ => { self.pos += 1; Some(Tokens::Pipe) }
                }
            },
            '(' => { self.pos += 1; Some(Tokens::LParentheses) },
            ')' => { self.pos += 1; Some(Tokens::RParentheses) },
            '{' => { self.pos += 1; Some(Tokens::LBrace) },
            '}' => { self.pos += 1; Some(Tokens::RBrace) },
            '[' => { self.pos += 1; Some(Tokens::LSquareBracket) },
            ']' => { self.pos += 1; Some(Tokens::RSquareBracket) },
            ',' => { self.pos += 1; Some(Tokens::Comma) },
            '$' => { self.pos += 1; Some(Tokens::Dollor) },
            '@' => { self.pos += 1; Some(Tokens::At) },
            '#' => { self.pos += 1; Some(Tokens::Hash) },
            '^' => { self.pos += 1; Some(Tokens::Caret) },
            '"' => self.tokenize_string_literal(),
            '\'' => self.tokenize_char_literal(),
            '0'..='9' => self.tokenize_number_literal(),
            'a'..='z' | 'A'..='Z' | '_' => self.tokenize_identifier_or_keyword(),
            _ => { self.pos += 1; None }, // Skip unknown characters
        }
    }

    fn tokenize_string_literal(&mut self) -> Option<Tokens> {
        self.pos += 1; // Skip the opening quote
        let mut literal = String::new();
        while self.pos < self.input.len() {
            let ch = self.input[self.pos..].chars().next()?;
            if ch == '"' {
                break; // Closing quote found
            } else if ch == '\\' {
                // Handle escape sequence
                self.pos += 1; // Skip the backslash
                if self.pos >= self.input.len() {
                    break; // Unexpected end of input
                }
                let escape_ch = self.input[self.pos..].chars().next()?;
                match escape_ch {
                    'n' => literal.push('\n'),
                    't' => literal.push('\t'),
                    'r' => literal.push('\r'),
                    '\\' => literal.push('\\'),
                    '"' => literal.push('"'),
                    '\'' => literal.push('\''),
                    _ => literal.push(escape_ch), // Unrecognized escape, treat as literal
                }
            } else {
                literal.push(ch);
            }
            self.pos += 1;
        }
        self.pos += 1; // Skip the closing quote
        Some(Tokens::StringLiteral(literal))
    }

    fn tokenize_identifier_or_keyword(&mut self) -> Option<Tokens> {
        let start = self.pos;
        while self.pos < self.input.len() && self.is_identifier_char(self.input[self.pos..].chars().next()?) {
            self.pos += 1;
        }
        let identifier = &self.input[start..self.pos];
        if let Some(builtin) = BaseFunctions::from_str(identifier) {
            Some(Tokens::BaseFunctions(builtin))
        } else if let Some(var_type) = VarType::from_str(identifier) {
            Some(Tokens::BaseVariables(var_type))
        } else if identifier == "true" {
            Some(Tokens::BoolLiteral(true))
        } else if identifier == "false" {
            Some(Tokens::BoolLiteral(false))
        } else {
            Some(Tokens::Identifier(identifier.to_string()))
        }
    }

    fn tokenize_number_literal(&mut self) -> Option<Tokens> {
        let start = self.pos;
        let mut has_decimal_point = false;

        while self.pos < self.input.len() {
            let ch = self.input[self.pos..].chars().next()?;
            if ch.is_digit(10) {
                self.pos += 1;
            } else if ch == '.' && !has_decimal_point {
                has_decimal_point = true;
                self.pos += 1;
            } else {
                break;
            }
        }

        let number_str = &self.input[start..self.pos];
        if has_decimal_point {
            number_str.parse::<f64>().ok().map(|float_value| Tokens::NumberLiteral(Numeric::Float(float_value)))
        } else {
            number_str.parse::<i64>().ok().map(|int_value| Tokens::NumberLiteral(Numeric::Int(int_value)))
        }
    }

    fn tokenize_char_literal(&mut self) -> Option<Tokens> {
        self.pos += 1; // Skip the opening single quote
        let ch = self.input[self.pos..].chars().next()?;
        self.pos += 1; // Move past the char
        if self.input[self.pos..].chars().next()? == '\'' {
            self.pos += 1; // Skip the closing single quote
            Some(Tokens::CharLiteral(ch))
        } else {
            None // Invalid char literal
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            match self.input[self.pos..].chars().next() {
                Some(ch) if ch.is_whitespace() => self.pos += 1,
                _ => break,
            }
        }
    }

    fn is_identifier_char(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn peek_char(&self, num: usize) -> Option<&str> {
        if self.pos + num <= self.input.len() {
            Some(&self.input[self.pos..self.pos + num])
        } else {
            None
        }
    }
}
