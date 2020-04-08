use crate::math::scanner::function::Function;
use crate::math::scanner::number::Number;
use crate::math::scanner::operator::UnaryOperator;
use crate::math::scanner::scanner::TokenState::{LValue, RValue};
use crate::math::scanner::text::Text;
use crate::math::scanner::token::Token;
use crate::scanner::cursor::Cursor;
use crate::scanner::{error, Scannable};
use alloc::vec::Vec;

enum TokenState {
    LValue,
    RValue,
}
enum ParenthesesState {
    SubExpression,
    FunctionArguments,
}
pub struct Scanner<'a> {
    cursor: Cursor<'a>,
    state: TokenState,
    parentheses_stack: Vec<ParenthesesState>,
}
impl<'a> Scanner<'a> {
    pub fn new(s: &'a str) -> Scanner<'a> {
        Scanner {
            cursor: Cursor::new(s),
            state: TokenState::LValue,
            parentheses_stack: Vec::new(),
        }
    }
    fn next_rvalue(&mut self) -> Result<Token<'a>, error::Error> {
        if self.state != RValue {
            return Err(error::Error::InvalidState);
        }
    }
    fn next_lvalue(&mut self) -> Result<Token<'a>, error::Error> {
        if self.state != LValue {
            return Err(error::Error::InvalidState);
        }
        let mut cursor = self.cursor.clone();
        cursor.skip_whitespace()?;
        let tok = match cursor.peek_char()? {
            '(' => {
                self.parentheses_stack.push(ParenthesesState::SubExpression);
                Token::LeftParentheses
            }
            '-' | '+' => Token::UnaryOperator(UnaryOperator::try_scan(&mut cursor)?),
            c if c.is_digit(10) => {
                self.state = TokenState::RValue;
                Token::Number(Number::try_scan(&mut cursor)?)
            }
            c if Text::is_text_char(c) => {
                let text = Text::try_scan(&mut cursor)?;
                if cursor.maybe_next_char('(').unwrap_or(false) {
                    self.parentheses_stack
                        .push(ParenthesesState::FunctionArguments);
                    Token::Function(Function(text))
                } else {
                    self.state = TokenState::RValue;
                    Token::Var(text)
                }
            }
            c => return Err(error::Error::UnexpectedCharacter(c)),
        };
    }
    pub fn next_token(&mut self) -> Option<Result<Token<'a>, error::Error>> {}
}
