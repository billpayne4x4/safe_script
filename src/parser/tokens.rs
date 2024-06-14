use crate::parser::dynamic_value::{ DynamicValue, VarType };

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BaseFunctions {
    If,
    Else,
    For,
    While,
    Switch,
    Case,
    Function,
    WriteLn,
    ConvertTo,
    // Add other built-in functions and keywords as needed
}

impl BaseFunctions {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "if" => Some(BaseFunctions::If),
            "else" => Some(BaseFunctions::Else),
            "for" => Some(BaseFunctions::For),
            "while" => Some(BaseFunctions::While),
            "switch" => Some(BaseFunctions::Switch),
            "case" => Some(BaseFunctions::Case),
            "fn" => Some(BaseFunctions::Function),
            "writeLn" => Some(BaseFunctions::WriteLn),
            "convertTo" => Some(BaseFunctions::ConvertTo),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Numeric {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Tokens {
    Identifier(String),		        // Identifier
    StringLiteral(String),	        // String literal
    NumberLiteral(Numeric),	        // Numeric literal
    BoolLiteral(bool),		        // Boolean literal
    CharLiteral(char),		        // Char literal
    Colon,  				        // :
    SemiColon,				        // ;
    Period,				            // .
    Equals,				            // =
    EqualEqual,				        // ==
    EqualPlus,  			        // =+
    EqualMinus, 			        // =-
    EqualStar,				        // =*
    EqualForwardSlash,		        // =/
    Plus,					        // +
    PlusPlus,   			        // ++
    PlusEqual,	    		        // +=
    Minus,					        // -
    MinusMinus,				        // --
    MinusEqual,				        // -=
    Bang,					        // !
    NotEqual,				        // !=
    Star,					        // *
    StarEqual,				        // *=
    ForwardSlash,			        // /
    ForwardSlashEqual, 		        // /=
    BackSlash, 				        // \
    Percent,				        // %
    Dollor,				            // $
    At,					            // @
    Hash,				            // #
    Caret,                          // ^
    LParentheses,			        // (
    RParentheses,			        // )
    LBrace,					        // {
    RBrace,					        // }
    LSquareBracket,			        // [
    RSquareBracket,			        // ]
    Comma,				            // ,
    Greater,				        // >
    GreaterEqual,			        // >=
    Less,					        // <
    LessEqual,				        // <=
    Ampersand,				        // &
    AndAnd,					        // &&
    Pipe,					        // |
    Or, 					        // ||
    BaseVariables(VarType),	        // Base variable types
    BaseFunctions(BaseFunctions),	// Base functions
}
