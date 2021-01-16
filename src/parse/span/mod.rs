mod tokenspan;

use crate::markdown::Span;
use nom::{
    branch::alt,
    error::{context, ContextError, Error, ErrorKind, ParseError},
    Err, IResult,
};
use tokenspan::{parse_three, parse_token, parse_token_strikeout, TokenSpan};

//解析文本
pub fn parse_span(i: &str) -> IResult<&str, Span> {
    context(
        "parse_span",
        alt((
            parse_span_strikeout,
            parse_span_bold_italic,
            parse_span_bold,
            parse_span_italic,
            parse_span_text,
        )),
    )(i)
}

//解析普通文本
pub fn parse_span_text(i: &str) -> IResult<&str, Span> {
    let mut token: Vec<TokenSpan> = Vec::new();

    let mut tmp = i;
    loop {
        let (i, o) = parse_token(tmp)?;

        match o {
            TokenSpan::Finish => {
                let s = token
                    .iter()
                    .fold(String::new(), |s, item| s + item.display());
                return Ok((i, Span::Text(s)));
            }
            _ => {
                token.push(o);
            }
        }

        tmp = i;
    }
}

//解析斜体字
pub fn parse_span_italic(i: &str) -> IResult<&str, Span> {
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
pub fn parse_span_bold(i: &str) -> IResult<&str, Span> {
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
pub fn parse_span_bold_italic(i: &str) -> IResult<&str, Span> {
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
pub fn parse_span_strikeout(i: &str) -> IResult<&str, Span> {
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
        parse_span, parse_span_bold, parse_span_bold_italic, parse_span_italic,
        parse_span_strikeout, parse_span_text,
    };
    use crate::markdown::Span;

    #[test]
    pub fn parse_span_test() {
        // let i = "11\n";
        // let i = "~11~\n";
        // let i = "***11***\n";
        // let i = "***11**\n";
        // let i = "***11*\n";
        // let i = "**11*\n";
        let i = "11*\n";
        let (i, o) = parse_span(i).unwrap();
        assert_eq!("\n", i);
        // assert_eq!(Span::Text("11".to_string()), o);
        // assert_eq!(Span::Strikeout("11".to_string()), o);
        // assert_eq!(Span::BoldItalic("11".to_string()), o);
        // assert_eq!(Span::Bold("*11".to_string()), o);
        // assert_eq!(Span::Italic("**11".to_string()), o);
        // assert_eq!(Span::Italic("*11".to_string()), o);
        assert_eq!(Span::Text("11*".to_string()), o);
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
