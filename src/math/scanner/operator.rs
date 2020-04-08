use crate::scanner::cursor::Cursor;
use crate::scanner::{error, Scannable};

pub struct InvalidOperatorError(());

pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}
impl BinaryOperator {
    pub fn which_operator(c: char) -> Option<BinaryOperator> {
        match c {
            '+' => Some(BinaryOperator::Plus),
            '-' => Some(BinaryOperator::Minus),
            '*' => Some(BinaryOperator::Multiply),
            '/' => Some(BinaryOperator::Divide),
            _ => None,
        }
    }
}
impl Scannable for BinaryOperator {
    fn try_scan(c: &mut Cursor) -> Result<Self, error::Error>
    where
        Self: Sized,
    {
        Self::which_operator(c.next_char()?).ok_or(error::Error::ExpectedCharacters("+-*/"))
    }
}

pub enum UnaryOperator {
    Plus,
    Minus,
}
impl UnaryOperator {
    pub fn which_operator(c: char) -> Option<UnaryOperator> {
        match c {
            '+' => Some(UnaryOperator::Plus),
            '-' => Some(UnaryOperator::Minus),
            _ => None,
        }
    }
}
impl Scannable for UnaryOperator {
    fn try_scan(c: &mut Cursor) -> Result<Self, error::Error>
    where
        Self: Sized,
    {
        Self::which_operator(c.next_char()?).ok_or(error::Error::ExpectedCharacters("+-"))
    }
}
