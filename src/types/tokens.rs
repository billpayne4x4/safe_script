use crate::types::var_type::VarType;
use crate::types::base_functions::BaseFunctions;
use crate::types::numeric::Numeric;

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
