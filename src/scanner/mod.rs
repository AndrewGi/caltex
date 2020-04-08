use crate::scanner::cursor::Cursor;

pub mod cursor;
pub mod error;

pub trait Scannable {
    fn try_scan(c: &mut Cursor) -> Result<Self, error::Error>
    where
        Self: Sized;
}
