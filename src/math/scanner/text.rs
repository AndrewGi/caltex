use crate::scanner::cursor::Cursor;
use crate::scanner::error::Error;
use crate::scanner::{error, Scannable};
use alloc::string::String;
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Text(pub String);
impl Text {
    const TEXT_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const TEXT_DIGITS_CHARS: &'static str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890";
    pub fn is_text_char(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
    pub fn is_text_char_or_digit(c: char) -> bool {
        Self::is_text_char(c) || c.is_digit(10)
    }
}

impl Scannable for Text {
    fn try_scan(c: &mut Cursor) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if Text::is_text_char(c.peek_char()?) {
            Ok(Text(
                c.capture_while(Text::is_text_char_or_digit)?
                    .ok_or(error::Error::ExpectedCharacters(Text::TEXT_DIGITS_CHARS))?
                    .to_owned(),
            ))
        } else {
            Err(Error::ExpectedCharacters(Self::TEXT_CHARS))
        }
    }
}
