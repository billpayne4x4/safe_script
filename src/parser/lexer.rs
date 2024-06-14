use crate::parser::tokens::{Tokens, BaseFunctions, Numeric};
use crate::parser::dynamic_value::{DynamicValue, VarType};

pub(crate) struct Lexer {
    input: String,
    pos: usize,
}

impl Lexer {
    pub(crate) fn new(script: &str) -> Self {
        Lexer {
            input: script.to_string(),
            pos: 0,
        }
    }

    pub(crate) fn tokenize(&mut self) -> Vec<Tokens> {
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

// #################################################################################################
// LEXER TESTS

#[test]
fn test_lexer_declare_int_variable() {
let script = "int x;";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Tokens::BaseVariables(VarType::Int));
    assert_eq!(tokens[1], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[2], Tokens::SemiColon);
}

#[test]
fn test_lexer_declare_int_variable_with_value() {
    let script = "int x = 100;";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Tokens::BaseVariables(VarType::Int));
    assert_eq!(tokens[1], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[2], Tokens::Equals);
    assert_eq!(tokens[3], Tokens::NumberLiteral(Numeric::Int(100)));
    assert_eq!(tokens[4], Tokens::SemiColon);
}

#[test]
fn test_lexer_declare_float_variable() {
    let script = "float x;";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Tokens::BaseVariables(VarType::Float));
    assert_eq!(tokens[1], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[2], Tokens::SemiColon);
}

#[test]
fn test_lexer_declare_float_variable_with_value() {
    let script = "float x = 100.123;";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Tokens::BaseVariables(VarType::Float));
    assert_eq!(tokens[1], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[2], Tokens::Equals);
    assert_eq!(tokens[3], Tokens::NumberLiteral(Numeric::Float(100.123)));
    assert_eq!(tokens[4], Tokens::SemiColon);
}

#[test]
fn test_lexer_if() {
    let script = "if (x >= 7) { int y; y = 15; }";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens[0], Tokens::BaseFunctions(BaseFunctions::If));
    assert_eq!(tokens[1], Tokens::LParentheses);
    assert_eq!(tokens[2], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[3], Tokens::GreaterEqual);
    assert_eq!(tokens[4], Tokens::NumberLiteral(Numeric::Int(7)));
    assert_eq!(tokens[5], Tokens::RParentheses);
    assert_eq!(tokens[6], Tokens::LBrace);
    assert_eq!(tokens[7], Tokens::BaseVariables(VarType::Int));
    assert_eq!(tokens[8], Tokens::Identifier("y".to_string()));
    assert_eq!(tokens[9], Tokens::SemiColon);
    assert_eq!(tokens[10], Tokens::Identifier("y".to_string()));
    assert_eq!(tokens[11], Tokens::Equals);
    assert_eq!(tokens[12], Tokens::NumberLiteral(Numeric::Int(15)));
    assert_eq!(tokens[13], Tokens::SemiColon);
    assert_eq!(tokens[14], Tokens::RBrace);
}

#[test]
fn test_lexer_if_else() {
    let script = "if (x) { int y; } else { string z = \"this is int *=; test\"}";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens[0], Tokens::BaseFunctions(BaseFunctions::If));
    assert_eq!(tokens[1], Tokens::LParentheses);
    assert_eq!(tokens[2], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[3], Tokens::RParentheses);
    assert_eq!(tokens[4], Tokens::LBrace);
    assert_eq!(tokens[5], Tokens::BaseVariables(VarType::Int));
    assert_eq!(tokens[6], Tokens::Identifier("y".to_string()));
    assert_eq!(tokens[7], Tokens::SemiColon);
    assert_eq!(tokens[8], Tokens::RBrace);
    assert_eq!(tokens[9], Tokens::BaseFunctions(BaseFunctions::Else));
    assert_eq!(tokens[10], Tokens::LBrace);
    assert_eq!(tokens[11], Tokens::BaseVariables(VarType::String));
    assert_eq!(tokens[12], Tokens::Identifier("z".to_string()));
    assert_eq!(tokens[13], Tokens::Equals);
    assert_eq!(tokens[14], Tokens::StringLiteral("this is int *=; test".to_string()));
    assert_eq!(tokens[15], Tokens::RBrace);
}

#[test]
fn test_while_with_spaces() {
    let script = "while (x != 10) { writeLn ($\"x equals {x}\"); x++; }";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens[0], Tokens::BaseFunctions(BaseFunctions::While));
    assert_eq!(tokens[1], Tokens::LParentheses);
    assert_eq!(tokens[2], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[3], Tokens::NotEqual);
    assert_eq!(tokens[4], Tokens::NumberLiteral(Numeric::Int(10)));
    assert_eq!(tokens[5], Tokens::RParentheses);
    assert_eq!(tokens[6], Tokens::LBrace);
    assert_eq!(tokens[7], Tokens::BaseFunctions(BaseFunctions::WriteLn));
    assert_eq!(tokens[8], Tokens::LParentheses);
    assert_eq!(tokens[9], Tokens::Dollor);
    assert_eq!(tokens[10], Tokens::StringLiteral("x equals {x}".to_string()));
    assert_eq!(tokens[11], Tokens::RParentheses);
    assert_eq!(tokens[12], Tokens::SemiColon);
    assert_eq!(tokens[13], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[14], Tokens::PlusPlus);
    assert_eq!(tokens[15], Tokens::SemiColon);
    assert_eq!(tokens[16], Tokens::RBrace);
}

#[test]
fn test_while_without_spaces() {
    // This test was added due to fix an issue with empty spaces in the script
    let script = "while(x != 10){writeLn($\"x equals {x}\");x++;}";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens[0], Tokens::BaseFunctions(BaseFunctions::While));
    assert_eq!(tokens[1], Tokens::LParentheses);
    assert_eq!(tokens[2], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[3], Tokens::NotEqual);
    assert_eq!(tokens[4], Tokens::NumberLiteral(Numeric::Int(10)));
    assert_eq!(tokens[5], Tokens::RParentheses);
    assert_eq!(tokens[6], Tokens::LBrace);
    assert_eq!(tokens[7], Tokens::BaseFunctions(BaseFunctions::WriteLn));
    assert_eq!(tokens[8], Tokens::LParentheses);
    assert_eq!(tokens[9], Tokens::Dollor);
    assert_eq!(tokens[10], Tokens::StringLiteral("x equals {x}".to_string()));
    assert_eq!(tokens[11], Tokens::RParentheses);
    assert_eq!(tokens[12], Tokens::SemiColon);
    assert_eq!(tokens[13], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[14], Tokens::PlusPlus);
    assert_eq!(tokens[15], Tokens::SemiColon);
    assert_eq!(tokens[16], Tokens::RBrace);
}

#[test]
fn test_for() {
    let script = "for (int i = 0; i <= 100; i++) { writeLn ($\"i equals {i}\"); }";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens[0], Tokens::BaseFunctions(BaseFunctions::For));
    assert_eq!(tokens[1], Tokens::LParentheses);
    assert_eq!(tokens[2], Tokens::BaseVariables(VarType::Int));
    assert_eq!(tokens[3], Tokens::Identifier("i".to_string()));
    assert_eq!(tokens[4], Tokens::Equals);
    assert_eq!(tokens[5], Tokens::NumberLiteral(Numeric::Int(0)));
    assert_eq!(tokens[6], Tokens::SemiColon);
    assert_eq!(tokens[7], Tokens::Identifier("i".to_string()));
    assert_eq!(tokens[8], Tokens::LessEqual);
    assert_eq!(tokens[9], Tokens::NumberLiteral(Numeric::Int(100)));
    assert_eq!(tokens[10], Tokens::SemiColon);
    assert_eq!(tokens[11], Tokens::Identifier("i".to_string()));
    assert_eq!(tokens[12], Tokens::PlusPlus);
    assert_eq!(tokens[13], Tokens::RParentheses);
    assert_eq!(tokens[14], Tokens::LBrace);
    assert_eq!(tokens[15], Tokens::BaseFunctions(BaseFunctions::WriteLn));
    assert_eq!(tokens[16], Tokens::LParentheses);
    assert_eq!(tokens[17], Tokens::Dollor);
    assert_eq!(tokens[18], Tokens::StringLiteral("i equals {i}".to_string()));
    assert_eq!(tokens[19], Tokens::RParentheses);
    assert_eq!(tokens[20], Tokens::SemiColon);
    assert_eq!(tokens[21], Tokens::RBrace);
}

#[test]
fn test_switch_case() {
    let script = "switch (x) { case 1: { k = 8; } case 2: { k-=; } }";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens[0], Tokens::BaseFunctions(BaseFunctions::Switch));
    assert_eq!(tokens[1], Tokens::LParentheses);
    assert_eq!(tokens[2], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[3], Tokens::RParentheses);
    assert_eq!(tokens[4], Tokens::LBrace);
    assert_eq!(tokens[5], Tokens::BaseFunctions(BaseFunctions::Case));
    assert_eq!(tokens[6], Tokens::NumberLiteral(Numeric::Int(1)));
    assert_eq!(tokens[7], Tokens::Colon);
    assert_eq!(tokens[8], Tokens::LBrace);
    assert_eq!(tokens[9], Tokens::Identifier("k".to_string()));
    assert_eq!(tokens[10], Tokens::Equals);
    assert_eq!(tokens[11], Tokens::NumberLiteral(Numeric::Int(8)));
    assert_eq!(tokens[12], Tokens::SemiColon);
    assert_eq!(tokens[13], Tokens::RBrace);
    assert_eq!(tokens[14], Tokens::BaseFunctions(BaseFunctions::Case));
    assert_eq!(tokens[15], Tokens::NumberLiteral(Numeric::Int(2)));
    assert_eq!(tokens[16], Tokens::Colon);
    assert_eq!(tokens[17], Tokens::LBrace);
    assert_eq!(tokens[18], Tokens::Identifier("k".to_string()));
    assert_eq!(tokens[19], Tokens::MinusEqual);
    assert_eq!(tokens[20], Tokens::SemiColon);
    assert_eq!(tokens[21], Tokens::RBrace);
    assert_eq!(tokens[22], Tokens::RBrace);
}

#[test]
fn test_function() {
    let script = "switch (x) { case 1: { k = 8; } case 2: { k-=; } }";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);

    assert_eq!(tokens[0], Tokens::BaseFunctions(BaseFunctions::Switch));
    assert_eq!(tokens[1], Tokens::LParentheses);
    assert_eq!(tokens[2], Tokens::Identifier("x".to_string()));
    assert_eq!(tokens[3], Tokens::RParentheses);
    assert_eq!(tokens[4], Tokens::LBrace);
    assert_eq!(tokens[5], Tokens::BaseFunctions(BaseFunctions::Case));
    assert_eq!(tokens[6], Tokens::NumberLiteral(Numeric::Int(1)));
    assert_eq!(tokens[7], Tokens::Colon);
    assert_eq!(tokens[8], Tokens::LBrace);
    assert_eq!(tokens[9], Tokens::Identifier("k".to_string()));
    assert_eq!(tokens[10], Tokens::Equals);
    assert_eq!(tokens[11], Tokens::NumberLiteral(Numeric::Int(8)));
    assert_eq!(tokens[12], Tokens::SemiColon);
    assert_eq!(tokens[13], Tokens::RBrace);
    assert_eq!(tokens[14], Tokens::BaseFunctions(BaseFunctions::Case));
    assert_eq!(tokens[15], Tokens::NumberLiteral(Numeric::Int(2)));
    assert_eq!(tokens[16], Tokens::Colon);
    assert_eq!(tokens[17], Tokens::LBrace);
    assert_eq!(tokens[18], Tokens::Identifier("k".to_string()));
    assert_eq!(tokens[19], Tokens::MinusEqual);
    assert_eq!(tokens[20], Tokens::SemiColon);
    assert_eq!(tokens[21], Tokens::RBrace);
    assert_eq!(tokens[22], Tokens::RBrace);
}

#[test]
fn test_base_functions() {
    let script = "if else for while switch case fn writeLn convertTo";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0], Tokens::BaseFunctions(BaseFunctions::If));
    assert_eq!(tokens[1], Tokens::BaseFunctions(BaseFunctions::Else));
    assert_eq!(tokens[2], Tokens::BaseFunctions(BaseFunctions::For));
    assert_eq!(tokens[3], Tokens::BaseFunctions(BaseFunctions::While));
    assert_eq!(tokens[4], Tokens::BaseFunctions(BaseFunctions::Switch));
    assert_eq!(tokens[5], Tokens::BaseFunctions(BaseFunctions::Case));
    assert_eq!(tokens[6], Tokens::BaseFunctions(BaseFunctions::Function));
    assert_eq!(tokens[7], Tokens::BaseFunctions(BaseFunctions::WriteLn));
    assert_eq!(tokens[8], Tokens::BaseFunctions(BaseFunctions::ConvertTo));
}

#[test]
fn test_numeric_literals() {
    let script = "123 45.67";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0], Tokens::NumberLiteral(Numeric::Int(123)));
    assert_eq!(tokens[1], Tokens::NumberLiteral(Numeric::Float(45.67)));
}

#[test]
fn test_string_literal() {
    let script = "\"Hello, World!\" \"Hello, World Again!\"";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0], Tokens::StringLiteral("Hello, World!".to_string()));
    assert_eq!(tokens[1], Tokens::StringLiteral("Hello, World Again!".to_string()));
}

#[test]
fn test_tokenize_string_literal_with_various_escapes() {
    let script = r#""Line 1\nLine 2\tTabbed\\Backslash\"Quote\"\rCarriageReturn\'SingleQuote""#;
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        Tokens::StringLiteral("Line 1\nLine 2\tTabbed\\Backslash\"Quote\"\rCarriageReturn\'SingleQuote".to_string())
    );
}

#[test]
fn test_string_literal_with_reserved_words() {
    let script = "\"fn this() { int x = 0; }\" \"Hello, World Yet Again!\"";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0], Tokens::StringLiteral("fn this() { int x = 0; }".to_string()));
    assert_eq!(tokens[1], Tokens::StringLiteral("Hello, World Yet Again!".to_string()));
}

#[test]
fn test_bool_literals() {
    let script = "true false true false false true false true";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0], Tokens::BoolLiteral(true));
    assert_eq!(tokens[1], Tokens::BoolLiteral(false));
    assert_eq!(tokens[2], Tokens::BoolLiteral(true));
    assert_eq!(tokens[3], Tokens::BoolLiteral(false));
    assert_eq!(tokens[4], Tokens::BoolLiteral(false));
    assert_eq!(tokens[5], Tokens::BoolLiteral(true));
    assert_eq!(tokens[6], Tokens::BoolLiteral(false));
    assert_eq!(tokens[7], Tokens::BoolLiteral(true));
}

#[test]
fn test_char_literal() {
    let script = "'a'";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0], Tokens::CharLiteral('a'));
}

#[test]
fn test_base_variables() {
    let script = "int float bool char byte";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0], Tokens::BaseVariables(VarType::Int));
    assert_eq!(tokens[1], Tokens::BaseVariables(VarType::Float));
    assert_eq!(tokens[2], Tokens::BaseVariables(VarType::Bool));
    assert_eq!(tokens[3], Tokens::BaseVariables(VarType::Char));
    assert_eq!(tokens[4], Tokens::BaseVariables(VarType::Byte));
}

#[test]
fn test_operators_and_punctuation() {
    let script = ": ; . = == =+ =- =* =/ + ++ += - -- -= ! != * / /= \\ % $ @ # ^ ( ) { } [ ] , > >= < <= & && | ||";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0], Tokens::Colon);
    assert_eq!(tokens[1], Tokens::SemiColon);
    assert_eq!(tokens[2], Tokens::Period);
    assert_eq!(tokens[3], Tokens::Equals);
    assert_eq!(tokens[4], Tokens::EqualEqual);
    assert_eq!(tokens[5], Tokens::EqualPlus);
    assert_eq!(tokens[6], Tokens::EqualMinus);
    assert_eq!(tokens[7], Tokens::EqualStar);
    assert_eq!(tokens[8], Tokens::EqualForwardSlash);
    assert_eq!(tokens[9], Tokens::Plus);
    assert_eq!(tokens[10], Tokens::PlusPlus);
    assert_eq!(tokens[11], Tokens::PlusEqual);
    assert_eq!(tokens[12], Tokens::Minus);
    assert_eq!(tokens[13], Tokens::MinusMinus);
    assert_eq!(tokens[14], Tokens::MinusEqual);
    assert_eq!(tokens[15], Tokens::Bang);
    assert_eq!(tokens[16], Tokens::NotEqual);
    assert_eq!(tokens[17], Tokens::Star);
    assert_eq!(tokens[18], Tokens::ForwardSlash);
    assert_eq!(tokens[19], Tokens::EqualForwardSlash);
    assert_eq!(tokens[20], Tokens::BackSlash);
    assert_eq!(tokens[21], Tokens::Percent);
    assert_eq!(tokens[22], Tokens::Dollor);
    assert_eq!(tokens[23], Tokens::At);
    assert_eq!(tokens[24], Tokens::Hash);
    assert_eq!(tokens[25], Tokens::Caret);
    assert_eq!(tokens[26], Tokens::LParentheses);
    assert_eq!(tokens[27], Tokens::RParentheses);
    assert_eq!(tokens[28], Tokens::LBrace);
    assert_eq!(tokens[29], Tokens::RBrace);
    assert_eq!(tokens[30], Tokens::LSquareBracket);
    assert_eq!(tokens[31], Tokens::RSquareBracket);
    assert_eq!(tokens[32], Tokens::Comma);
    assert_eq!(tokens[33], Tokens::Greater);
    assert_eq!(tokens[34], Tokens::GreaterEqual);
    assert_eq!(tokens[35], Tokens::Less);
    assert_eq!(tokens[36], Tokens::LessEqual);
    assert_eq!(tokens[37], Tokens::Ampersand);
    assert_eq!(tokens[38], Tokens::AndAnd);
    assert_eq!(tokens[39], Tokens::Pipe);
    assert_eq!(tokens[40], Tokens::Or);
}

#[test]
fn test_identifiers() {
    let script = "variableName anotherVar _privateVar var0123";
    let mut lexer = Lexer::new(script);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0], Tokens::Identifier("variableName".to_string()));
    assert_eq!(tokens[1], Tokens::Identifier("anotherVar".to_string()));
    assert_eq!(tokens[2], Tokens::Identifier("_privateVar".to_string()));
    assert_eq!(tokens[3], Tokens::Identifier("var0123".to_string()));
}
