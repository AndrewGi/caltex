use crate::scanner::error;
use core::str::Chars;

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    s: Chars<'a>,
}
impl<'a> Cursor<'a> {
    pub fn new(s: &'a str) -> Cursor<'a> {
        Cursor { s: s.chars() }
    }
    pub fn skip_whitespace(&mut self) -> Result<(), error::EOLError> {
        while self.peek_char()?.is_whitespace() {
            self.next_char()?;
        }
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.s.clone().next().is_none()
    }
    pub fn byte_offset(&self, second: &Self) -> usize {
        second.s.as_str().as_ptr() as usize - self.s.as_str().as_ptr() as usize
    }
    pub fn next_char(&mut self) -> Result<char, error::EOLError> {
        self.s.next().ok_or(error::EOLError(()))
    }
    pub fn peek_char(&self) -> Result<char, error::EOLError> {
        self.s.clone().next().ok_or(error::EOLError(()))
    }
    pub fn expect_char(&mut self, c: char) -> Result<(), error::Error> {
        if self.next_char()? == c {
            Ok(())
        } else {
            Err(error::Error::ExpectedCharacter(c))
        }
    }
    pub fn maybe_next_char(&mut self, c: char) -> Result<bool, error::EOLError> {
        if self.peek_char()? == c {
            self.next_char()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn capture_while(
        &mut self,
        mut f: impl FnMut(char) -> bool,
    ) -> Result<Option<&'a str>, error::EOLError> {
        let start = self.clone();
        if f(self.peek_char()?) {
            self.next_char()?;
        } else {
            return Ok(None);
        }
        while let Ok(c) = self.peek_char() {
            if f(c) {
                self.next_char()?;
            } else {
                return Ok(Some(
                    start.str_span(self).expect("objects from same cursor"),
                ));
            }
        }
        Ok(Some(
            start.str_span(self).expect("objects from same cursor"),
        ))
    }
    pub fn capture_digits(&mut self) -> Result<Option<&'a str>, error::EOLError> {
        self.capture_while(|c| c.is_digit(10))
    }
    pub fn str_span(&self, other: &Self) -> Option<&'a str> {
        let byte_offset = self.byte_offset(other);
        if byte_offset > self.s.as_str().len() {
            None
        } else {
            Some(&self.s.as_str()[..byte_offset])
        }
    }
}
