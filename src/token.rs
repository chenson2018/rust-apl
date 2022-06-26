use crate::apl_type::AplType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub literal: Option<AplType>,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

/// An enum for representing the allowed tokens in APL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // glyphs
    LeftArrow,
    Plus,
    Minus,
    Divide,
    Star,
    Log,
    Domino,
    Circle,
    ExclamationMark,
    QuestionMark,
    Stile,
    Upstile,
    Downstile,
    UpTack,
    DownTack,
    LeftTack,
    RightTack,
    Equal,
    NotEqual,
    LessThanEqual,
    LessThan,
    GreaterThan,
    GreaterThanEqual,
    EqualUnderbar,
    EqualUnderbarSlash,
    Or,
    And,
    Nor,
    Nand,
    UpArrow,
    DownArrow,
    LeftShoe,
    RightShoe,
    LeftShoeUnderbar,
    Squad,
    GradeUp,
    GradeDown,
    Iota,
    IotaUnderbar,
    Epsilon,
    EpsilonUnderbar,
    DownShoe,
    UpShoe,
    Tilde,
    Slash,
    SlashBar,
    Backslash,
    BackslashBar,
    Comma,
    CommaBar,
    Rho,
    CircleStile,
    CircleBar,
    Transpose,
    Diaeresis,
    TildeDiaeresis,
    StarDiaeresis,
    Dot,
    Jot,
    JotDiaeresis,
    CircleDiaeresis,
    At,
    QuoteQuad,
    Quad,
    QuadColon,
    QuadEqual,
    QuadDiamond,
    Ibeam,
    Hydrant,
    Thorn,
    Diamond,
    Lamp,
    RightArrow,
    Omega,
    Alpha,
    Del,
    Ampersand,
    HighMinus,
    Zilde,

    // non glyphs
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Newline,

    // literals
    String,
    Identifier,
    Number,

    Eof,
}

lazy_static! {
    pub static ref TOKENS: HashMap<char, TokenType> = {
        let mut m = HashMap::new();
        m.insert('←', TokenType::LeftArrow);
        m.insert('+', TokenType::Plus);
        m.insert('-', TokenType::Minus);
        m.insert('÷', TokenType::Divide);
        m.insert('*', TokenType::Star);
        m.insert('⍟', TokenType::Log);
        m.insert('⌹', TokenType::Domino);
        m.insert('○', TokenType::Circle);
        m.insert('!', TokenType::ExclamationMark);
        m.insert('?', TokenType::QuestionMark);
        m.insert('|', TokenType::Stile);
        m.insert('⌈', TokenType::Upstile);
        m.insert('⌊', TokenType::Downstile);
        m.insert('⊥', TokenType::UpTack);
        m.insert('⊤', TokenType::DownTack);
        m.insert('⊣', TokenType::LeftTack);
        m.insert('⊢', TokenType::RightTack);
        m.insert('=', TokenType::Equal);
        m.insert('≠', TokenType::NotEqual);
        m.insert('≤', TokenType::LessThanEqual);
        m.insert('<', TokenType::LessThan);
        m.insert('>', TokenType::GreaterThan);
        m.insert('≥', TokenType::GreaterThanEqual);
        m.insert('≡', TokenType::EqualUnderbar);
        m.insert('≢', TokenType::EqualUnderbarSlash);
        m.insert('∨', TokenType::Or);
        m.insert('∧', TokenType::And);
        m.insert('⍱', TokenType::Nor);
        m.insert('⍲', TokenType::Nand);
        m.insert('↑', TokenType::UpArrow);
        m.insert('↓', TokenType::DownArrow);
        m.insert('⊂', TokenType::LeftShoe);
        m.insert('⊃', TokenType::RightShoe);
        m.insert('⊆', TokenType::LeftShoeUnderbar);
        m.insert('⌷', TokenType::Squad);
        m.insert('⍋', TokenType::GradeUp);
        m.insert('⍒', TokenType::GradeDown);
        m.insert('⍳', TokenType::Iota);
        m.insert('⍸', TokenType::IotaUnderbar);
        m.insert('∊', TokenType::Epsilon);
        m.insert('⍷', TokenType::EpsilonUnderbar);
        m.insert('∪', TokenType::DownShoe);
        m.insert('∩', TokenType::UpShoe);
        m.insert('~', TokenType::Tilde);
        m.insert('/', TokenType::Slash);
        m.insert('\\', TokenType::Backslash);
        m.insert('⌿', TokenType::SlashBar);
        m.insert('⍀', TokenType::BackslashBar);
        m.insert(',', TokenType::Comma);
        m.insert('⍪', TokenType::CommaBar);
        m.insert('⍴', TokenType::Rho);
        m.insert('⌽', TokenType::CircleStile);
        m.insert('⊖', TokenType::CircleBar);
        m.insert('⍉', TokenType::Transpose);
        m.insert('¨', TokenType::Diaeresis);
        m.insert('⍨', TokenType::TildeDiaeresis);
        m.insert('⍣', TokenType::StarDiaeresis);
        m.insert('.', TokenType::Dot);
        m.insert('∘', TokenType::Jot);
        m.insert('⍤', TokenType::JotDiaeresis);
        m.insert('⍥', TokenType::CircleDiaeresis);
        m.insert('@', TokenType::At);
        m.insert('⍞', TokenType::QuoteQuad);
        m.insert('⎕', TokenType::Quad);
        m.insert('⍠', TokenType::QuadColon);
        m.insert('⌸', TokenType::QuadEqual);
        m.insert('⌺', TokenType::QuadDiamond);
        m.insert('⌶', TokenType::Ibeam);
        m.insert('⍎', TokenType::Hydrant);
        m.insert('⍕', TokenType::Thorn);
        m.insert('⋄', TokenType::Diamond);
        m.insert('→', TokenType::RightArrow);
        m.insert('⍵', TokenType::Omega);
        m.insert('⍺', TokenType::Alpha);
        m.insert('∇', TokenType::Del);
        m.insert('&', TokenType::Ampersand);
        m.insert('⍬', TokenType::Zilde);
        m.insert('(', TokenType::LeftParenthesis);
        m.insert(')', TokenType::RightParenthesis);
        m.insert('{', TokenType::LeftBrace);
        m.insert('}', TokenType::RightBrace);
        m
    };
}
