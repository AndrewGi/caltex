use crate::math::scanner::function::Function;
use crate::math::scanner::number::Number;
use crate::math::scanner::operator::BinaryOperator;
use crate::math::scanner::operator::UnaryOperator;
use crate::math::scanner::text::Text;

#[derive(Clone, PartialOrd, PartialEq, Debug)]
pub enum Token {
    Comma,
    LeftParentheses,
    RightParentheses,
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
    Number(Number),
    Var(Text),
    Function(Function),
}
