use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    combinator::{map, not, peek},
    error::context,
    sequence::preceded,
    IResult,
};

use crate::markdown::Span;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenSpan {
    Bold,
    Italic,
    BoldItalic,
    Strikeout,
    Finish,
    Text(String),
}

impl TokenSpan {
    pub fn display(&self) -> &str {
        match self {
            TokenSpan::Bold => "**",
            TokenSpan::Italic => "*",
            TokenSpan::BoldItalic => "***",
            TokenSpan::Strikeout => "~",
            TokenSpan::Text(s) => s,
            _ => "",
        }
    }
}

pub fn parse_token(i: &str) -> IResult<&str, TokenSpan> {
    context(
        "parse_token",
        alt((
            parse_token_text,
            parse_token_finish,
            parse_token_strikeout,
            parse_three,
        )),
    )(i)
}

pub fn parse_three(i: &str) -> IResult<&str, TokenSpan> {
    context(
        "parse_three",
        alt((
            parse_token_bold_italic,
            parse_token_bold,
            parse_token_italic,
        )),
    )(i)
}

pub fn parse_token_text(i: &str) -> IResult<&str, TokenSpan> {
    context(
        "parse_token_text",
        map(
            take_till1(|c: char| c == '\n' || c == '*' || c == '~'),
            |s: &str| TokenSpan::Text(s.to_string()),
        ),
    )(i)
}

//解析Finish
pub fn parse_token_finish(i: &str) -> IResult<&str, TokenSpan> {
    context(
        "parse_token_finish",
        map(peek(tag("\n")), |_| TokenSpan::Finish),
    )(i)
}

//解析Strikeout
pub fn parse_token_strikeout(i: &str) -> IResult<&str, TokenSpan> {
    context(
        "parse_token_strikeout",
        map(tag("~"), |_| TokenSpan::Strikeout),
    )(i)
}

//解析BoldItalic
pub fn parse_token_bold_italic(i: &str) -> IResult<&str, TokenSpan> {
    context(
        "parse_token_bold_italic",
        map(tag("*"), |_| TokenSpan::BoldItalic),
    )(i)
}

//解析Italic
pub fn parse_token_italic(i: &str) -> IResult<&str, TokenSpan> {
    context("parse_token_italic", map(tag("**"), |_| TokenSpan::Italic))(i)
}

//解析Bold
pub fn parse_token_bold(i: &str) -> IResult<&str, TokenSpan> {
    context("parse_token_bold", map(tag("***"), |_| TokenSpan::Bold))(i)
}

pub fn parse(i: &str) -> IResult<&str, &str> {
    alt((tag("***"), tag("**"), tag("*")))(i)
}

mod test {
    use super::{
        parse, parse_token_bold, parse_token_bold_italic, parse_token_finish, parse_token_italic,
        parse_token_strikeout, parse_token_text, TokenSpan,
    };

    #[test]
    fn parse_token_strikeout_test() {
        let i = "~";
        let (i, o) = parse_token_strikeout(i).unwrap();
        assert_eq!(TokenSpan::Strikeout, o);
        assert_eq!("", i);
    }

    #[test]
    fn parse_token_text_test() {
        // let i = "1\n";
        // let i = "1*";
        let i = "1~";
        let (i, o) = parse_token_text(i).unwrap();
        assert_eq!(TokenSpan::Text("1".to_string()), o);
        // assert_eq!("\n", i);
        // assert_eq!("*", i);
        assert_eq!("~", i);
    }

    #[test]
    fn parse_token_finish_test() {
        let i = "\n";
        let (i, o) = parse_token_finish(i).unwrap();
        assert_eq!(TokenSpan::Finish, o);
        assert_eq!("\n", i);
    }

    #[test]
    fn parse_token_bold_italic_test() {
        let i = "*";
        let (i, o) = parse_token_bold_italic(i).unwrap();
        assert_eq!(TokenSpan::BoldItalic, o);
        assert_eq!("", i);
    }

    #[test]
    fn parse_token_italic_test() {
        let i = "**";
        let (i, o) = parse_token_italic(i).unwrap();
        assert_eq!(TokenSpan::Italic, o);
        assert_eq!("", i);
    }

    #[test]
    fn parse_token_bold_test() {
        let i = "***";
        let (i, o) = parse_token_bold(i).unwrap();
        assert_eq!(TokenSpan::Bold, o);
        assert_eq!("", i);
    }

    #[test]
    fn parse_test() {
        let i = "***";
        let (i, _o) = parse(i).unwrap();
        assert_eq!("", i);
    }
}
