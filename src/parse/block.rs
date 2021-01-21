use super::span::{parse_span, parse_span_link, parse_span_link_url, parse_span_text};
use crate::markdown::{Block, Span};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    combinator::{all_consuming, map, peek},
    error::context,
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

pub enum TokenBlock {
    Line(Vec<Span>),
    Hr,
}

//解析引用
pub fn parse_block_quote(i: &str) -> IResult<&str, Block> {
    context(
        "parse_block_quote",
        map(preceded(tag("> "), parse_block_quote_context), |item| {
            Block::Quote(item)
        }),
    )(i)
}

fn parse_block_quote_context(i: &str) -> IResult<&str, Vec<Block>> {
    unimplemented!()
}
//解析图片
pub fn parse_block_image(i: &str) -> IResult<&str, Block> {
    context(
        "parse_block_image",
        map(
            tuple((
                parse_block_image_alt,
                parse_span_link_url,
                parse_block_finish,
            )),
            |(r1, (r2, r3), _)| Block::Image(r1, r2, r3),
        ),
    )(i)
}

pub fn parse_block_image_alt(i: &str) -> IResult<&str, String> {
    context(
        "parse_block_image_alt",
        map(
            delimited(
                tag("!["),
                take_till1(|c: char| c == ']' || c == '\n'),
                tag("]"),
            ),
            |s: &str| s.to_string(),
        ),
    )(i)
}

//解析换行符
pub fn parse_block_hr(i: &str) -> IResult<&str, Block> {
    context("parse_block_hr", map(peek(tag("\n")), |_| Block::Hr))(i)
}

//解析段落
pub fn pares_block_paragraph(i: &str) -> IResult<&str, Block> {
    context(
        "pares_block_paragraph",
        map(terminated(many1(parse_span), parse_block_finish), |res| {
            Block::Paragraph(res)
        }),
    )(i)
}

//解析标题
pub fn parse_block_header(i: &str) -> IResult<&str, Block> {
    context(
        "parse_block_header",
        map(
            terminated(
                tuple((parse_block_header_level, parse_block_header_span)),
                parse_block_finish,
            ),
            |(num, vec)| Block::Header(num, vec),
        ),
    )(i)
}

fn parse_block_header_span(i: &str) -> IResult<&str, Vec<Span>> {
    context(
        "parse_block_header_span",
        many1(alt((parse_span_link, parse_span_text))),
    )(i)
}

fn parse_block_header_level(i: &str) -> IResult<&str, usize> {
    context(
        "parse_block_header_level",
        map(
            terminated(take_till1(|c: char| c != '#'), tag(" ")),
            |s: &str| s.len(),
        ),
    )(i)
}

//结尾或者遇到/n
fn parse_block_finish(i: &str) -> IResult<&str, ()> {
    context(
        "parse_block_finish",
        map(alt((all_consuming(tag("")), peek(tag("\n")))), |_| ()),
    )(i)
}

mod test {}
