use std::ops::Range;
use std::iter::Scan;

pub struct Scanner<'a> {
    input: &'a str,
}

pub struct Text<'a>(&'a str);
pub struct Group<'a>(Vec<Token<'a>>);
pub struct Argument<'a>(Token<'a>);
pub struct Command<'a> {
    command: &'a str,
    subscript: Token<'a>,
    arguments: Vec<Argment<'a>>
}
pub enum Token<'a> {
    Text(Text<'a>),
    Optional(Optional),
}
impl<'a> std::ops::Index<Range<usize>> for Scanner<'a> {
    type Output = Option<Scanner<'a>>;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        if self.range().contains(&index.end) && self.range().contains(&index.start) {
            Some(Scanner { self.input[]})
        }
    }
}
impl Scanner {
    pub fn peek(&self) -> Option<char> {
        if self.pos == self.input.len() {
            None
        } else {
            input[self.pos]
        }
    }
    pub fn next(&mut self) -> Option<char> {
        if Some(c) = self.peek() {
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }
    pub fn range(&self) -> Range<usize> {
        0 .. self.pos
    }
}

impl<'a> Scanner<'a> {
    pub fn next_group(&mut self) -> Option<Group<'a>> {

    }
}