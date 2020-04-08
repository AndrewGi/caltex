use crate::math::scanner::function::Function;
use crate::math::scanner::number::Number;
use crate::math::scanner::operator::UnaryOperator;
use crate::math::scanner::text::Text;
use crate::scanner::cursor::Cursor;
use crate::scanner::error::Error;
use crate::scanner::Scannable;

pub enum Token<'a> {
    Comma,
    LeftParentheses,
    RightParentheses,
    UnaryOperator(UnaryOperator),
    Number(Number),
    Var(Text<'a>),
    Function(Function<'a>),
}
