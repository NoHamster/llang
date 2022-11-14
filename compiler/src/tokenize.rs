use std::fs::File;
use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
enum input_token
{
    #[token("u8")]
    #[token("u16")]
    #[token("u32")]
    #[token("u64")]
    #[token("char")]
    #[token("f32")]
    #[oken("f64")]
    Type(llang_type),

    #[regex("[_a-zA-Z]+[_a-zA-Z0-9]*")]
    Name(String),

    #[regex("\"(^[\\\"]*\\.)*\"")]
    StrLitteral(String),

    #[token(":")]
    TagDef,

    #[token(",")]
    Comma,

    #[token("\n")]
    NewLine,

    #[regex("#[^\n]", logos::skip)]
    #[error]
    Error

}

fn lex_file(path: String) -> logos::Lexer<input_token>
{
    let input = std::fs::
}
