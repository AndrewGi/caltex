use std::ops::Range;
use std::iter::Scan;
use std::str::Chars;
use crate::math::basic_number::BasicNumber;
use crate::math::num::Number;
use crate::latex::scanner::Brackets::{Curly, Square};

#[derive(Clone)]
pub struct Scanner<'a> {
    chars: Chars<'a>,
}

pub struct Text<'a>(&'a str);
#[derive(Default, Clone)]
pub struct Group<'a>(Vec<Token<'a>>);
pub struct Argument<'a>(Token<'a>);
pub struct SquareBrackets<'a>(Group<'a>);
pub struct CurlyBrackets<'a>(Group<'a>);
pub struct Parentheses<'a>(Group<'a>);
pub struct Escaped(char);
pub enum Brackets<'a>{
    Square(SquareBrackets<'a>),
    Curly(CurlyBrackets<'a>)
}
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
    Number(BasicNumber),
    Group(Group<'a>),
    Text(Text<'a>),
}
impl Operator {
    pub fn is_operator(c: char) -> bool {
        match c {
            '+' | '-' | '/' | '*' | '|' | '&' | '%' | '$' => true,
            _ => false
        }
    }
}
impl Scanner {
    pub fn new(s: &str) -> Scanner {
        Self {
            chars: s.chars()
        }
    }
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
    pub fn next_number(&mut self) -> Option<BasicNumber> {

    }
    pub fn next_command(&mut self) -> Option<Command<'a>> {
        let backup = self.clone();
        if self.peek()? != '\\' {
            // Missing start '\' before command.
            return None
        }
        // Capture the slash.
        self.next().unwrap();
        let command_name = self.next_text()?.0;
        let mut brackets = vec![];
        while let
    }
    pub fn next_escaped(&mut self) -> Option<Escaped> {

    }
    pub fn all_to_group(&mut self) -> Option<Group<'a>> {
        if self.peek()?.is_whitespace() {
            self.eat_whitespace();
        }
        let mut out = Group::default();
        while self.peek().is_some() {
            out.0.push(self.next_token()?);
            self.eat_whitespace();
        }
        Some(out)
    }
    pub fn next_bracket(&mut self) -> Option<Brackets<'a>> {
        self.eat_whitespace();
        if self.peek()? == '{' {
            self.next_group(('{', '}')).map(|x| Curly(CurlyBrackets(x)) )
        } else if self.peek()? == '[' {
            self.next_group(('[', ']')).map(|x| Square(SquareBrackets(x)) )
        } else {
            None
        }
    }
    /// Consumes the stream between the two delimiters. Assumes Scanner is place at the first delimiter.
    ///
    pub fn next_group(&mut self, delimiters: (char, char)) -> Option<Group<'a>> {
        if self.peek()? != delimiters.0 {
            return None;
        }
        let start = self.clone();
        self.next()?; // Consume the first delimiter
        let mut depth = 1;
        let mut char_count = 0;
        while depth > 0 {
            let c = self.next()?;
            char_count += c.len_utf8();
            if c == delimiters.0 {
                // Start delimiter
                depth += 1;
            } else if c == delimiters.1 {
                // End delimiter
                depth -= 1;
            }
        }
        Scanner::new(&start.chars.as_str()[..char_count]).all_to_group()
    }
}