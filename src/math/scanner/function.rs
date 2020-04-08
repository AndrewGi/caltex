use crate::math::scanner::text::Text;
use crate::scanner::cursor::Cursor;
use crate::scanner::error::Error;
use crate::scanner::Scannable;

pub struct Function<'a>(pub Text<'a>);
impl<'a> Function<'a> {}
impl<'a> Scannable for Function {
    fn try_scan(c: &mut Cursor<'a>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let name = Text::try_scan(c)?;
        c.expect_char('(')?;
        Ok(Function(name))
    }
}
