use crate::scanner::cursor::Cursor;
use crate::scanner::error::Error;
use crate::scanner::{error, Scannable};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Text<'a>(pub &'a str);
impl<'a> Text<'a> {
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

impl<'a> Scannable for Text<'a> {
    fn try_scan(c: &mut Cursor<'a>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if Text::is_text_char(c.peek_char()?) {
            Ok(Text(c.capture_while(Text::is_text_char_or_digit)?.ok_or(
                error::Error::ExpectedCharacters(Text::TEXT_DIGITS_CHARS),
            )?))
        } else {
            Error::ExpectedCharacters(Self::TEXT_CHARS)
        }
    }
}
