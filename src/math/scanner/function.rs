use crate::math::scanner::text::Text;
use crate::scanner::cursor::Cursor;
use crate::scanner::error::Error;
use crate::scanner::Scannable;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct Function(pub Text);
impl Scannable for Function {
    fn try_scan(c: &mut Cursor) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let name = Text::try_scan(c)?;
        c.expect_char('(')?;
        Ok(Function(name))
    }
}
