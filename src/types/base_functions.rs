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