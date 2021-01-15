pub struct Markdown(Vec<Block>);

pub enum Block {}

#[derive(Debug, PartialEq)]
pub enum Span {
    //粗体文本
    Bold(String),
    //斜体
    Italic(String),
    //斜体加粗
    BoldItalic(String),
    //删除线，
    Strikeout(String),
}
