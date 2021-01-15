mod tokenspan;

use crate::markdown::Span;
use nom::{
    error::{ContextError, Error, ErrorKind, ParseError},
    Err, IResult,
};
use tokenspan::*;

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
    use super::parse_span_strikeout;
    use crate::markdown::Span;

    #[test]
    pub fn parse_span_strikeout_test() {
        let i = "~11~";
        let (i, o) = parse_span_strikeout(i).unwrap();
        assert_eq!(Span::Strikeout("11".to_string()), o);
        assert_eq!("", i);
    }
}
