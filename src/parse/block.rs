use super::span::{parse_span_link, parse_span_text};
use crate::markdown::{Block, Span};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    combinator::{all_consuming, map, peek},
    error::context,
    multi::many1,
    sequence::terminated,
    IResult,
};

// pub fn parse_block_paragraph(i: &str) -> IResult<&str, Block> {
//     context("parse_block_paragraph", map(first, second))
// }

pub fn parse_block_paragraph_span(i: &str) -> IResult<&str, Vec<Span>> {
    context(
        "parse_block_paragraph_span",
        many1(alt((parse_span_link, parse_span_text))),
    )(i)
}

pub fn parse_block_paragraph_level(i: &str) -> IResult<&str, usize> {
    context(
        "parse_block_paragraph_level",
        map(
            terminated(take_till1(|c: char| c != '#'), tag(" ")),
            |s: &str| s.len(),
        ),
    )(i)
}

//结尾或者遇到/n
pub fn parse_block_finish(i: &str) -> IResult<&str, ()> {
    context(
        "parse_block_finish",
        map(alt((all_consuming(tag("")), peek(tag("\n")))), |_| ()),
    )(i)
}

mod test {
    use super::{parse_block_finish, parse_block_paragraph_level, parse_block_paragraph_span};
    use crate::markdown::Span;

    // #[test]
    // fn parse_block_paragraph_span_test() {
    //     let i = "111\n";
    //     let (i, o) = parse_block_paragraph_span(i).unwrap();
    //     assert_eq!(vec![Span::Text("11".to_string())], o);
    //     assert_eq!("11", i);
    // }

    #[test]
    fn parse_block_paragraph_level_test() {
        // let i = "# 11";
        let i = "######## 11";
        let (i, o) = parse_block_paragraph_level(i).unwrap();
        // assert_eq!(1, o);
        assert_eq!(8, o);
        assert_eq!("11", i);
    }

    #[test]
    fn parse_block_finish_test() {
        let i = "\n";
        let (i, o) = parse_block_finish(i).unwrap();
        assert_eq!("\n", i);
    }
}
