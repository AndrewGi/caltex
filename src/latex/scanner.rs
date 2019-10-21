use std::ops::Range;
use std::iter::Scan;
use std::str::Chars;

pub struct Scanner<'a> {
    chars: Chars<'a>,
}

pub struct Number
pub struct Text<'a>(&'a str);
pub struct Group<'a>(Vec<Token<'a>>);
pub struct Argument<'a>(Token<'a>);
pub struct Command<'a> {
    command: &'a str,
    subscript: Token<'a>,
    arguments: Vec<Argument<'a>>
}
pub struct Operator(char);
pub struct Subscript<'a> {
    parent: Token<'a>,
    script: Token<'a>
}
pub struct Superscript<'a> {
    parent: Token<'a>,
    script: Token<'a>
}
pub enum Token<'a> {
    Subscript(Subscript<'a>),
    Superscript(Superscript<'a>),
    Group(Group<'a>),
    Text(Text<'a>),
}
impl Operator {
    pub fn is_operator(c: char) -> bool {
        match c {
            '+' | '-' | '/' | '*' | '|' | '&' | '%' => true,
            _ => false
        }
    }
}
impl Scanner {
    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }
    pub fn eat_whitespace(&mut self) {
        while self.peek().unwrap_or('_').is_whitespace() {
            self.next()
        }
    }
    pub fn len(&self) -> usize {
        self.chars.as_str().len()
    }
}

impl<'a> Scanner<'a> {
    pub fn next_token(&mut self) -> Option<Token<'a>> {

    }
    pub fn next_text(&mut self) -> Option<Text<'a>> {
        let start = self.chars.as_str();
        let mut size = 0;
        while self.peek().unwrap_or(' ').is_alphabetic() {
            size+=self.next().unwrap().len_utf8();
        }
        Some(Text(&start[..size]))
    }
    pub fn next_command(&mut self) -> Option<Command<'a>> {
        if self.peek()? != '\\' {
            // Missing start '\' before command.
            return None
        }
        // Capture the slash.
        self.next().unwrap();
        let command_name = self.next_text()?.0;
        if self.peek()? == '[' {
            // There are options.
        }
        let arguments = vec![];
        while self.peek()? == '{' {
            arguments.push(Argument(self.next_group(('{','}'))?));
        }
    }
    pub fn next_group(&mut self, delimiters: Option<(char, char)>) -> Option<Group<'a>> {
        if self.peek()?
    }
}