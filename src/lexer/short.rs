#[macro_export]
macro_rules! T {
    [+] => {
        $crate::lexer::TokenKind::Plus
    };
    [-] => {
        $crate::lexer::TokenKind::Minus
    };
    [*] => {
        $crate::lexer::TokenKind::Times
    };
    [/] => {
        $crate::lexer::TokenKind::Slash
    };
    [^] => {
        $crate::lexer::TokenKind::Pow
    };
    [=] => {
        $crate::lexer::TokenKind::Eq
    };
    [.] => {
        $crate::lexer::TokenKind::Dot
    };
    [,] => {
        $crate::lexer::TokenKind::Comma
    };
    [_] => {
        $crate::lexer::TokenKind::Underscore
    };
    [!] => {
        $crate::lexer::TokenKind::Bang
    };
    [&] => {
        $crate::lexer::TokenKind::Ampersand
    };
    [|] => {
        $crate::lexer::TokenKind::Bar
    };
    [:] => {
        $crate::lexer::TokenKind::Colon
    };
    [;] => {
        $crate::lexer::TokenKind::SemiColon
    };
    [<] => {
        $crate::lexer::TokenKind::LAngle
    };
    [>] => {
        $crate::lexer::TokenKind::RAngle
    };
    ['['] => {
        $crate::lexer::TokenKind::LSquare
    };
    [']'] => {
        $crate::lexer::TokenKind::RSquare
    };
    ['{'] => {
        $crate::lexer::TokenKind::LBrace
    };
    ['}'] => {
        $crate::lexer::TokenKind::RBrace
    };
    ['('] => {
        $crate::lexer::TokenKind::LParen
    };
    [')'] => {
        $crate::lexer::TokenKind::RParen
    };
    [string] => {
        $crate::lexer::TokenKind::String
    };
    [comment] => {
        $crate::lexer::TokenKind::Comment
    };
    [int] => {
        $crate::lexer::TokenKind::Int
    };
    [float] => {
        $crate::lexer::TokenKind::Float
    };
    [ident] => {
        $crate::lexer::TokenKind::Identifier
    };
    [let] => {
        $crate::lexer::TokenKind::KeywordLet
    };
    [fn] => {
        $crate::lexer::TokenKind::KeywordFn
    };
    [struct] => {
        $crate::lexer::TokenKind::KeywordStruct
    };
    [if] => {
        $crate::lexer::TokenKind::KeywordIf
    };
    [else] => {
        $crate::lexer::TokenKind::KeywordElse
    };
    [&&] => {
        $crate::lexer::TokenKind::And
    };
    [||] => {
        $crate::lexer::TokenKind::Or
    };
    [==] => {
        $crate::lexer::TokenKind::Eqq
    };
    [!=] => {
        $crate::lexer::TokenKind::Neq
    };
    [>=] => {
        $crate::lexer::TokenKind::Geq
    };
    [<=] => {
        $crate::lexer::TokenKind::Leq
    };
    [error] => {
        $crate::lexer::TokenKind::Error
    };
    [ws] => {
        $crate::lexer::TokenKind::Whitespace
    };
    [EOF] => {
        $crate::lexer::TokenKind::Eof
    };
}

