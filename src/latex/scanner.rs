use std::ops::Range;
use std::iter::Scan;
use std::str::Chars;
use crate::math::basic_number::BasicNumber;
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
    subscript: Option<Subscript<'a>>,
    superscript: Option<Superscript<'a>>,
    brackets: Vec<Brackets<'a>>
}
pub struct Operator(char);
pub struct Script<'a> {
    c: char,
    script: Token<'a>
}
pub struct Subscript<'a> {
    script: Token<'a>
}
pub struct Superscript<'a> {
    script: Token<'a>
}
pub enum Token<'a> {
    Command(Command<'a>),
    Subscript(Subscript<'a>),
    Superscript(Superscript<'a>),
    Number(BasicNumber),
    Bracket(Brackets<'a>),
    Text(Text<'a>),
    Operator(Operator),
}
impl Operator {
    pub fn is_operator(c: char) -> bool {
        match c {
            '+' | '-' | '/' | '*' | '|' | '&' | '%' | '$' => true,
            _ => false
        }
    }
}
impl Subscript {
    pub fn new(script: Token<'_>) -> Subscript<'_> {
        Subscript {
            script
        }
    }
}
impl Superscript {
    pub fn new(script: Token<'_>) -> Superscript<'_> {
        Superscript {
            script
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
        let mut clone = self.clone();
        clone.eat_whitespace();
        let next_token = match clone.peek()? {
            '\\' => Token::Command(clone.next_command()?),
            '{' | '[' => Token::Bracket(clone.next_bracket()?),
            '-' => {
                match clone.next_number() {
                    Some(n) => Token::Number(n),
                    None => clone.next_operator(),
                }
            },
            '_' => Token::Subscript(clone.next_subscript()?),
            '^' => Token::Superscript(clone.next_superscript()?),
            c if Operator::is_operator(c) => {
                Token::Operator(clone.next_operator()?)
            },
            c if c.is_alphabetic() => {
                Token::Text(clone.next_text()?)
            }
            '.' => Token::Number(clone.next_number()?),
            c if c.is_numeric() => {
              Token::Number(clone.next_number()?)
            },
            _ => unimplemented!()
        };
        *self = clone;
        Some(next_token)
    }
    pub fn next_script(&mut self, script_char: char) -> Option<Script<'a>> {
        if self.peek()? == script_char {
            self.next().unwrap(); // Consume '_' or '^'
            Some(Script {
                c: script_char,
                script: self.next_token()?
            })
        } else {
            None
        }
    }
    pub fn next_superscript(&mut self) -> Option<Superscript<'a>> {
        self.next_script('^').map(|s| Superscript::new(s.script))
    }
    pub fn next_subscript(&mut self) -> Option<Subscript<'a>> {
        self.next_script('_').map(|s| Subscript::new(s.script))
    }
    pub fn next_operator(&mut self) -> Option<Operator> {
        if Operator::is_operator(self.peek()?) {
            Some(Operator(self.next().unwrap()))
        } else {
            None
        }
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
        let start = self.chars.as_str();
        let mut size = 0;
        let is_neg = self.peek()? == '-';
        if is_neg {
            size += '-'.len_utf8();
            self.next().unwrap();
        }
        while self.peek().unwrap_or('~').is_numeric() {
            size += self.next().unwrap().len_utf8();
        }
        if self.peek() == '.' {
            size += '.'.len_utf8();
            self.next().unwrap();
        }
        while self.peek().unwrap_or('~').is_numeric() {
            size += self.next().unwrap().len_utf8();
        }
        (&start[..size]).parse().ok()
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
        let mut sub_script = if self.peek() == '_' {
            self.next_subscript()
        } else {
            None
        };
        let mut super_script = if self.peek() == '_' {
            self.next_superscript()
        } else {
            None
        };
        if sub_script.is_none() && self.peek()? == '_' {
            sub_script = self.next_subscript();
        }
        let mut brackets = vec![];
        while let Some(bracket) = self.next_bracket() {
            brackets.push(bracket);
        }
        Some(Command {
            command: command_name,
            subscript: sub_script,
            superscript: super_script,
            brackets: brackets
        })
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