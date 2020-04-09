use crate::math::basic_number::BasicNumber;
use crate::scanner::cursor::Cursor;
use crate::scanner::error::Error;
use crate::scanner::Scannable;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Number(pub BasicNumber);

impl Scannable for Number {
    fn try_scan(c: &mut Cursor) -> Result<Self, Error>
    where
        Self: Sized,
    {
        const DIGITS: &str = "0123456789";
        let start: Cursor = (*c).clone();
        let start_digits = c
            .capture_digits()?
            .ok_or(Error::ExpectedCharacters(DIGITS))?;
        if c.maybe_next_char('.').unwrap_or(false) {
            // Float
            if c.peek_char().is_ok() {
                let _end_digits = c.capture_digits()?;
            }
            let s = start.str_span(c).expect("cursors from same object");
            Ok(Number(BasicNumber::Float(
                s.parse::<f64>().expect("characters already checked"),
            )))
        } else {
            // Integer
            Ok(Number(BasicNumber::Int(
                start_digits
                    .parse::<i64>()
                    .expect("characters already checked"),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::error;
    fn test_scan_number(input: &str, expected: BasicNumber) -> Result<(), error::Error> {
        let mut c = Cursor::new(input);
        let n = Number::try_scan(&mut c)?;
        assert_eq!(n.0, expected);
        assert!(c.peek_char().is_err(), "didn't scan the whole input");
        Ok(())
    }
    #[test]
    fn test_scan() -> Result<(), error::Error> {
        test_scan_number("1.23", BasicNumber::Float(1.23))?;
        test_scan_number("2.25", BasicNumber::Float(2.25))?;
        test_scan_number("231.", BasicNumber::Float(231.0))?;
        test_scan_number("1", BasicNumber::Int(1))?;
        test_scan_number("1123", BasicNumber::Int(1123))?;
        test_scan_number("1123312", BasicNumber::Int(1123312))?;
        let mut c = Cursor::new("0.99*");
        let n = Number::try_scan(&mut c)?;
        assert_eq!(n.0, BasicNumber::Float(0.99));
        assert!(c.peek_char().is_ok(), "should have more characters");
        Ok(())
    }
}
