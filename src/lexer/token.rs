use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq)]
enum Token {
    // Mathematical
    #[token("*")]
    Multiply,
    #[token("+")]
    Add,
    #[token("-")]
    Minus,
    #[token("/")]
    Divide,
    #[token("^")]
    Pow,
    // General
    #[token("=")]
    Equals,
    #[token(".")]
    Period,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token(",")]
    Comma,
    #[token("_")]
    Underscore,
    // Boolean
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<=")]
    Leq,
    #[token(">=")]
    Geq,
    // Brackets
    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    // Constructs
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,
    #[regex(r#"//[^\n]*\n"#)]
    LineComment,
    #[regex(r#"\d+"#, priority = 2)]
    Int,
    #[regex(r#"((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#)]
    Float,
    #[regex(r#"[A-Za-z]([A-Za-z]|_|\d)*"#)]
    Ident,

    // Keywords
    #[token("let")]
    KwLet,
    #[token("if")]
    KwIf,
    #[token("else")]
    KwElse,
    #[token("fn")]
    KwFn,
    #[token("struct")]
    KwStruct,

    #[regex(r"[ \t\r\n\f]+")]
    WS
}


