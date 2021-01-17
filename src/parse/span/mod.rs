mod tokenspan;

use crate::markdown::Span;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    combinator::map,
    error::{context, ContextError, Error, ErrorKind, ParseError},
    multi::many1,
    sequence::{delimited, separated_pair, tuple},
    Err, IResult,
};
use tokenspan::{parse_three, parse_token, parse_token_strikeout, parse_token_text, TokenSpan};

//解析文本
pub fn parse_span(i: &str) -> IResult<&str, Span> {
    context(
        "parse_span",
        alt((
            parse_span_link,
            parse_span_strikeout,
            parse_span_bold_italic,
            parse_span_bold,
            parse_span_italic,
            parse_span_text,
        )),
    )(i)
}

//解析超链接
pub fn parse_span_link(i: &str) -> IResult<&str, Span> {
    context(
        "parse_span_link",
        map(
            tuple((parse_span_link_text, parse_span_link_url)),
            |(r1, (r2, r3))| Span::Link(r1, r2, r3),
        ),
    )(i)
}

//解析超链接的超链接名
fn parse_span_link_text(i: &str) -> IResult<&str, String> {
    context(
        "parse_span_link_text",
        map(
            delimited(
                tag("["),
                take_till1(|c: char| c == ']' || c == '\n'),
                tag("]"),
            ),
            |s: &str| s.to_string(),
        ),
    )(i)
}

//解析超链接的url
fn parse_span_link_url(i: &str) -> IResult<&str, (String, Option<String>)> {
    context(
        "parse_span_link_url",
        alt((parse_span_url_title, parse_span_url_not_title)),
    )(i)
}

fn parse_span_url_title(i: &str) -> IResult<&str, (String, Option<String>)> {
    context(
        "parse_span_url_title",
        map(
            delimited(
                tag("("),
                separated_pair(
                    take_till1(|c: char| c == ' ' || c == '\n'),
                    tag(" "),
                    take_till1(|c: char| c == ')' || c == '\n'),
                ),
                tag(")"),
            ),
            |(r1, r2): (&str, &str)| (r1.to_string(), Some(r2.to_string())),
        ),
    )(i)
}

fn parse_span_url_not_title(i: &str) -> IResult<&str, (String, Option<String>)> {
    context(
        "parse_span_url_not_title",
        map(
            delimited(
                tag("("),
                take_till1(|c: char| c == ')' || c == '\n'),
                tag(")"),
            ),
            |s: &str| (s.to_string(), None),
        ),
    )(i)
}

//解析普通文本
pub fn parse_span_text(i: &str) -> IResult<&str, Span> {
    let (i, o) = parse_token_text(i)?;
    if let TokenSpan::Text(s) = o {
        return Ok((i, Span::Text(s)));
    } else {
        let e = Error::from_error_kind(i, ErrorKind::Tag);
        let e = Error::add_context(i, "parse_span_text", e);
        return Err(Err::Error(e));
    }
}

//解析斜体字
fn parse_span_italic(i: &str) -> IResult<&str, Span> {
    let (i, o) = parse_three(i)?;
    if !o.is_possible_italic() {
        let e = Error::from_error_kind(i, ErrorKind::Tag);
        let e = Error::add_context(i, "parse_span_italic", e);
        return Err(Err::Error(e));
    }

    let mut token: Vec<TokenSpan> = Vec::new();
    token.push(o);

    let mut tmp = i;
    loop {
        let (i, o) = parse_token(tmp)?;

        match o {
            TokenSpan::Italic | TokenSpan::Bold | TokenSpan::BoldItalic => {
                token.push(o);

                let s = token
                    .iter()
                    .fold(String::new(), |s, item| s + item.display_italic());
                return Ok((i, Span::Italic(s)));
            }
            TokenSpan::Finish => {
                let e = Error::from_error_kind(i, ErrorKind::Tag);
                let e = Error::add_context(i, "parse_span_italic", e);
                return Err(Err::Error(e));
            }
            _ => {
                token.push(o);
            }
        }

        tmp = i;
    }
}

//解析粗体文本
fn parse_span_bold(i: &str) -> IResult<&str, Span> {
    let (i, o) = parse_three(i)?;
    if !o.is_possible_bold() {
        let e = Error::from_error_kind(i, ErrorKind::Tag);
        let e = Error::add_context(i, "parse_span_bold", e);
        return Err(Err::Error(e));
    }

    let mut token: Vec<TokenSpan> = Vec::new();
    token.push(o);

    let mut tmp = i;
    loop {
        let (i, o) = parse_token(tmp)?;

        match o {
            TokenSpan::BoldItalic | TokenSpan::Bold => {
                token.push(o);

                let s = token
                    .iter()
                    .fold(String::new(), |s, item| s + item.display_bold());
                return Ok((i, Span::Bold(s)));
            }
            TokenSpan::Finish => {
                let e = Error::from_error_kind(i, ErrorKind::Tag);
                let e = Error::add_context(i, "parse_span_bold", e);
                return Err(Err::Error(e));
            }
            _ => {
                token.push(o);
            }
        }

        tmp = i;
    }
}

//解析斜体加粗的文本
fn parse_span_bold_italic(i: &str) -> IResult<&str, Span> {
    let (i, o) = parse_three(i)?;
    if TokenSpan::BoldItalic != o {
        let e = Error::from_error_kind(i, ErrorKind::Tag);
        let e = Error::add_context(i, "parse_span_bold_italic", e);
        return Err(Err::Error(e));
    }

    let mut token: Vec<TokenSpan> = Vec::new();

    let mut tmp = i;
    loop {
        let (i, o) = parse_token(tmp)?;

        match o {
            TokenSpan::BoldItalic => {
                let s = token
                    .iter()
                    .fold(String::new(), |s, item| s + item.display());
                return Ok((i, Span::BoldItalic(s)));
            }
            TokenSpan::Finish => {
                let e = Error::from_error_kind(i, ErrorKind::Tag);
                let e = Error::add_context(i, "parse_span_strikeout_finish", e);
                return Err(Err::Error(e));
            }
            _ => {
                token.push(o);
            }
        }

        tmp = i;
    }
}

//解析删除线文本
fn parse_span_strikeout(i: &str) -> IResult<&str, Span> {
    let (i, o) = parse_token_strikeout(i)?;
    if TokenSpan::Strikeout != o {
        let e = Error::from_error_kind(i, ErrorKind::Tag);
        let e = Error::add_context(i, "parse_span_strikeout", e);
        return Err(Err::Error(e));
    }

    let mut token: Vec<TokenSpan> = Vec::new();

    let mut tmp = i;
    loop {
        let (i, o) = parse_token(tmp)?;

        match o {
            TokenSpan::Strikeout => {
                let s = token
                    .iter()
                    .fold(String::new(), |s, item| s + item.display());
                return Ok((i, Span::Strikeout(s)));
            }
            TokenSpan::Finish => {
                let e = Error::from_error_kind(i, ErrorKind::Tag);
                let e = Error::add_context(i, "parse_span_strikeout_finish", e);
                return Err(Err::Error(e));
            }
            _ => {
                token.push(o);
            }
        }

        tmp = i;
    }
}

mod test {
    use super::{
        parse_span, parse_span_bold, parse_span_bold_italic, parse_span_italic, parse_span_link,
        parse_span_strikeout, parse_span_text, test,
    };
    use crate::markdown::Span;

    #[test]
    pub fn parse_span_link_test() {
        // let i = "[123](http://jianshu.com)";
        let i = "[123](http://baidu.com 111)";
        let (i, o) = parse_span_link(i).unwrap();
        assert_eq!("", i);
        // assert_eq!(
        //     Span::Link("123".to_string(), "http://baidu.com".to_string(), None),
        //     o
        // );
        assert_eq!(
            Span::Link(
                "123".to_string(),
                "http://baidu.com".to_string(),
                Some("111".to_string())
            ),
            o
        );
    }

    #[test]
    pub fn parse_span_test() {
        // let i = "11\n";
        // let i = "~11~\n";
        // let i = "***11***\n";
        // let i = "***11**\n";
        // let i = "***11*\n";
        // let i = "**11*\n";
        // let i = "11*\n";
        let i = "[123](http://baidu.com 111)\n";
        let (i, o) = parse_span(i).unwrap();
        assert_eq!("\n", i);
        // assert_eq!(Span::Text("11".to_string()), o);
        // assert_eq!(Span::Strikeout("11".to_string()), o);
        // assert_eq!(Span::BoldItalic("11".to_string()), o);
        // assert_eq!(Span::Bold("*11".to_string()), o);
        // assert_eq!(Span::Italic("**11".to_string()), o);
        // assert_eq!(Span::Italic("*11".to_string()), o);
        // assert_eq!(Span::Text("11*".to_string()), o);
        assert_eq!(
            Span::Link(
                "123".to_string(),
                "http://baidu.com".to_string(),
                Some("111".to_string())
            ),
            o
        );
    }

    #[test]
    pub fn parse_span_text_test() {
        let i = "11\n";
        let (i, o) = parse_span_text(i).unwrap();
        assert_eq!("\n", i);
        assert_eq!(Span::Text("11".to_string()), o);
    }

    #[test]
    pub fn parse_span_italic_test() {
        // let i = "**11**";
        // let i = "*11**";
        // let i = "**11*";
        // let i = "*11*";
        // let i = "*11***";
        let i = "***11*";
        let (i, o) = parse_span_italic(i).unwrap();
        assert_eq!("", i);
        // assert_eq!(Span::Italic("11*".to_string()), o);
        // assert_eq!(Span::Italic("*11".to_string()), o);
        assert_eq!(Span::Italic("**11".to_string()), o);
    }

    #[test]
    pub fn parse_span_bold_test() {
        // let i = "***11***";
        // let i = "***11**";
        // let i = "**11***";
        let i = "**11**";
        // let i = "**11\n**";
        let (i, o) = parse_span_bold(i).unwrap();
        assert_eq!("", i);
        // assert_eq!(Span::Bold("*11*".to_string()), o);
        // assert_eq!(Span::Bold("*11".to_string()), o);
        // assert_eq!(Span::Bold("11*".to_string()), o);
        assert_eq!(Span::Bold("11".to_string()), o);
    }

    #[test]
    pub fn parse_span_bold_italic_test() {
        let i = "***11***";
        let (i, o) = parse_span_bold_italic(i).unwrap();
        assert_eq!("", i);
        assert_eq!(Span::BoldItalic("11".to_string()), o);
    }

    #[test]
    pub fn parse_span_strikeout_test() {
        let i = "~11~";
        let (i, o) = parse_span_strikeout(i).unwrap();
        assert_eq!(Span::Strikeout("11".to_string()), o);
        assert_eq!("", i);
    }
}
