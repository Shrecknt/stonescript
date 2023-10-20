use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TOKENS_MAP: HashMap<&'static str, TokenTypeSpecific> = {
        let mut m = HashMap::new();
        m.insert("+", TokenTypeSpecific::Add);
        m.insert("-", TokenTypeSpecific::Subtract);
        m.insert("*", TokenTypeSpecific::Multiply);
        m.insert("/", TokenTypeSpecific::Divide);
        m.insert("%", TokenTypeSpecific::Modulo);
        m.insert("<", TokenTypeSpecific::LessThan);
        m.insert(">", TokenTypeSpecific::GreaterThan);
        m.insert("=", TokenTypeSpecific::Assignment);
        m.insert("!", TokenTypeSpecific::Not);
        m.insert("{", TokenTypeSpecific::OpenScope);
        m.insert("}", TokenTypeSpecific::CloseScope);
        m.insert("(", TokenTypeSpecific::OpenGroup);
        m.insert(")", TokenTypeSpecific::CloseGroup);
        m.insert("[", TokenTypeSpecific::OpenIndex);
        m.insert("]", TokenTypeSpecific::CloseIndex);
        m.insert(".", TokenTypeSpecific::Property);
        m.insert("?", TokenTypeSpecific::Ternary);
        m.insert(":", TokenTypeSpecific::TernarySplit);
        m.insert("=", TokenTypeSpecific::Equals);
        m.insert("<=", TokenTypeSpecific::LessThanEquals);
        m.insert(">=", TokenTypeSpecific::GreaterThanEquals);
        m.insert("!=", TokenTypeSpecific::NotEquals);
        m.insert("&&", TokenTypeSpecific::And);
        m.insert("||", TokenTypeSpecific::Or);
        m.insert("??", TokenTypeSpecific::NullishCoalescing);
        m.insert(";", TokenTypeSpecific::End);
        m.insert(",", TokenTypeSpecific::Separator);
        m.insert("->", TokenTypeSpecific::Lambda);
        m
    };
    pub static ref TOKEN_TYPE_SPECIFIC_MAP: HashMap<&'static str, TokenTypeSpecific> = {
        let mut m = HashMap::new();
        m.insert("literal", TokenTypeSpecific::Literal);
        m.insert("generic", TokenTypeSpecific::Generic);
        m.insert("signed_float", TokenTypeSpecific::SignedFloat);
        m.insert("signed_double", TokenTypeSpecific::SignedDouble);
        m.insert("signed_short", TokenTypeSpecific::SignedShort);
        m.insert("signed_int", TokenTypeSpecific::SignedInt);
        m.insert("signed_long", TokenTypeSpecific::SignedLong);
        m.insert("unsigned_short", TokenTypeSpecific::UnsignedShort);
        m.insert("unsigned_int", TokenTypeSpecific::UnsignedInt);
        m.insert("unsigned_long", TokenTypeSpecific::UnsignedLong);
        m
    };
    pub static ref KEYWORDS_MAP: HashMap<&'static str, Keyword> = {
        let mut m = HashMap::new();
        m.insert("for", Keyword::For);
        m.insert("while", Keyword::While);
        m.insert("let", Keyword::Let);
        m.insert("const", Keyword::Const);
        m.insert("function", Keyword::Function);
        m.insert("as", Keyword::As);
        m.insert("null", Keyword::Null);
        m.insert("return", Keyword::Return);
        m.insert("throw", Keyword::Throw);
        m
    };
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Token,
    String,
    Word,
    Number,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenTypeSpecific {
    Literal,
    Generic,
    SignedFloat,
    SignedDouble,
    SignedShort,
    SignedInt,
    SignedLong,
    UnsignedShort,
    UnsignedInt,
    UnsignedLong,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LessThan,
    GreaterThan,
    Assignment,
    Not,
    OpenScope,
    CloseScope,
    OpenGroup,
    CloseGroup,
    OpenIndex,
    CloseIndex,
    Property,
    Ternary,
    TernarySplit,
    Equals,
    LessThanEquals,
    GreaterThanEquals,
    NotEquals,
    And,
    Or,
    NullishCoalescing,
    End,
    Separator,
    Lambda,
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    For,
    While,
    Let,
    Const,
    Function,
    As,
    Null,
    Return,
    Throw,
}

pub const MAX_TOKEN_LENGTH: usize = 2;
