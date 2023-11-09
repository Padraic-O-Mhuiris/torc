use core::fmt;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq)]
enum Token {
    // Programmatic
    #[token("=")]
    Equals,

    // Mathematical
    #[token("+")]
    Plus,
    #[token("-")]
    Hyphen,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Solidus,
    #[token(r"\")]
    ReverseSolidus,
    #[token("^")]
    Circumflex,

    // Boolean
    #[token("==")]
    LogicalEquals,
    #[token("!=")]
    LogicalNotEquals,
    #[token("<")]
    LogicalLessThan,
    #[token(">")]
    LogicalGreaterThan,
    #[token("<=")]
    LogicalLessThanOrEquals,
    #[token(">=")]
    LogicalGreaterThanOrEquals,
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    
    // Punctuation
    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token("_")]
    Underscore,
        
    // Bracket Delimiters
    #[token("(")]
    LeftParenthesis,
    #[token(")")]
    RightParenthesis,
    #[token("{")]
    LeftCurlyBracket,
    #[token("}")]
    RightCurlyBracket,
    #[token("[")]
    LeftSquareBracket,
    #[token("]")]
    RightSquareBracket,

    // Data types
    #[regex(r#"\d+"#, priority = 2)]
    Integer,
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,
    #[regex(r#"((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#)]
    Float,

    #[regex(r#"[A-Za-z]([A-Za-z]|_|\d)*"#)]
    Identifier,

    // Misc
    #[regex(r"[ \t\r\n\f]+")]
    Whitespace,
}

#[macro_export]
macro_rules! T {
    [=] => { Token::Equals };

    [+] => { Token::Plus };
    [-] => { Token::Hyphen };
    [*] => { Token::Asterisk };
    [/] => { Token::Solidus };
    [r"\"] => { Token::ReverseSolidus };
    [^] => { Token::Circumflex };
    
    [==] => { Token::LogicalEquals };
    [!=] => { Token::LogicalNotEquals };
    [<] => { Token::LogicalLessThan };
    [>] => { Token::LogicalGreaterThan };
    [<=] => { Token::LogicalLessThanOrEquals };
    [>=] => { Token::LogicalGreaterThanOrEquals };
    [&&] => { Token::LogicalAnd };
    [||] => { Token::LogicalOr };

    [.] => { Token::Period };
    [,] => { Token::Comma };
    [:] => { Token::Colon };
    [;] => { Token::SemiColon };
    [_] => { Token::Underscore };

    ['('] => { Token::LeftParenthesis };
    [')'] => { Token::RightParenthesis };
    ['{'] => { Token::LeftCurlyBracket };
    ['}'] => { Token::RightCurlyBracket };
    ['['] => { Token::LeftSquareBracket };
    [']'] => { Token::RightSquareBracket };
    
    [int] => { Token::Integer };
    [string] => { Token::String };
    [float] => { Token::Float };
    [ident] => { Token::Identifier };

    [ws] => { Token::Whitespace };
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T![=] => "=",

                T![+] => "+",
                T![-] => "-",
                T![*] => "*",
                T![/] => "/",
                T![r"\"] => r"\",
                T![^] => "^",
                
                T![==] => "==",
                T![!=] => "!=",
                T![<] => "<",
                T![>] => ">",
                T![<=] => "<=",
                T![>=] => ">=",
                T![&&] => "&&",
                T![||] => "||",
                
                T![.] => ".",
                T![,] => ",",
                T![:] => ":",
                T![;] => ";",
                T![_] => "_",

                T!['('] => "(",
                T![')'] => ")",
                T!['{'] => "{",
                T!['}'] => "}",
                T!['['] => "[",
                T![']'] => "]",
                    
                T![int] => "Integer",
                T![string] => "String",
                T![float] => "Float",
                T![ident] => "Identifier",
                
                T![ws] => "<Whitespace>",
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_display() {
        assert_eq!(T![=].to_string(), "=");
        
        assert_eq!(T![+].to_string(), "+");
        assert_eq!(T![-].to_string(), "-");
        assert_eq!(T![*].to_string(), "*");
        assert_eq!(T![/].to_string(), "/");
        assert_eq!(T![r"\"].to_string(), r"\");
        assert_eq!(T![^].to_string(), "^");

        assert_eq!(T![==].to_string(), "==");
        assert_eq!(T![!=].to_string(), "!=");
        assert_eq!(T![<].to_string(), "<");
        assert_eq!(T![>].to_string(), ">");
        assert_eq!(T![<=].to_string(), "<=");
        assert_eq!(T![&&].to_string(), "&&");
        assert_eq!(T![||].to_string(), "||");

        assert_eq!(T![.].to_string(), ".");
        assert_eq!(T![,].to_string(), ",");
        assert_eq!(T![:].to_string(), ":");
        assert_eq!(T![;].to_string(), ";");
        assert_eq!(T![_].to_string(), "_");
        
        assert_eq!(T!['('].to_string(), "(");
        assert_eq!(T![')'].to_string(), ")");
        assert_eq!(T!['{'].to_string(), "{");
        assert_eq!(T!['}'].to_string(), "}");
        assert_eq!(T!['['].to_string(), "[");
        assert_eq!(T![']'].to_string(), "]");
        
        assert_eq!(T![int].to_string(), "Integer");
        assert_eq!(T![string].to_string(), "String");
        assert_eq!(T![float].to_string(), "Float");
        assert_eq!(T![ident].to_string(), "Identifier");
        
        assert_eq!(T![ws].to_string(), "<Whitespace>");
   }
}
