use crate::math::scanner::function::Function;
use crate::math::scanner::number::Number;
use crate::math::scanner::operator::{BinaryOperator, UnaryOperator};
use crate::math::scanner::text::Text;
use crate::math::scanner::token::Token;
use crate::scanner::cursor::Cursor;
use crate::scanner::{error, Scannable};
use alloc::vec::Vec;
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum TokenState {
    LValue,
    RValue,
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum ParenthesesState {
    SubExpression,
    FunctionArguments,
}
#[derive(Debug, Clone)]
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
    fn next_rvalue(&mut self) -> Result<Token, error::Error> {
        if self.state != TokenState::RValue {
            return Err(error::Error::InvalidState);
        }
        let mut cursor = self.cursor.clone();
        let para_state = self.parentheses_stack.last().copied();
        cursor.skip_whitespace()?;
        let tok = match (cursor.peek_char()?, para_state) {
            (')', Some(_)) => {
                cursor.next_char()?;
                self.parentheses_stack
                    .pop()
                    .expect("this should only fail if allow_right_paranthese is incorrect");
                Token::RightParentheses
            }
            (',', Some(ParenthesesState::FunctionArguments)) => {
                cursor.next_char()?;
                self.state = TokenState::LValue;
                Token::Comma
            }
            (c, _) => {
                if let Some(op) = BinaryOperator::which_operator(c) {
                    cursor.next_char()?;
                    self.state = TokenState::LValue;
                    Token::BinaryOperator(op)
                } else {
                    return Err(error::Error::UnexpectedCharacter(c));
                }
            }
        };
        self.cursor = cursor;
        Ok(tok)
    }
    fn next_lvalue(&mut self) -> Result<Token, error::Error> {
        if self.state != TokenState::LValue {
            return Err(error::Error::InvalidState);
        }
        let mut cursor = self.cursor.clone();
        cursor.skip_whitespace()?;
        let tok = match cursor.peek_char()? {
            '(' => {
                cursor.next_char()?;
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
        self.cursor = cursor;
        Ok(tok)
    }
    pub fn next_token(&mut self) -> Option<Result<Token, error::Error>> {
        if self.cursor.is_empty() {
            None
        } else {
            Some(match self.state {
                TokenState::LValue => self.next_lvalue(),
                TokenState::RValue => self.next_rvalue(),
            })
        }
    }
}
impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, error::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::math::basic_number::BasicNumber;
    use crate::scanner::error;
    fn expect_token(s: &mut Scanner, tok: Token) {
        assert_eq!(s.next(), Some(Ok(tok)), "scanner: {:?}", s);
    }
    #[test]
    fn test1() -> Result<(), error::Error> {
        let mut scanner = Scanner::new("10 * 2.5");
        expect_token(&mut scanner, Token::Number(Number(BasicNumber::Int(10))));
        expect_token(
            &mut scanner,
            Token::BinaryOperator(BinaryOperator::Multiply),
        );
        expect_token(&mut scanner, Token::Number(Number(BasicNumber::Float(2.5))));
        assert_eq!(scanner.next(), None);
        Ok(())
    }
    #[test]
    fn test2() -> Result<(), error::Error> {
        let mut scanner = Scanner::new("(10 * 2.5)");
        expect_token(&mut scanner, Token::LeftParentheses);
        expect_token(&mut scanner, Token::Number(Number(BasicNumber::Int(10))));
        expect_token(
            &mut scanner,
            Token::BinaryOperator(BinaryOperator::Multiply),
        );
        expect_token(&mut scanner, Token::Number(Number(BasicNumber::Float(2.5))));
        expect_token(&mut scanner, Token::RightParentheses);
        assert_eq!(scanner.next(), None);

        Ok(())
    }
    #[test]
    fn test3() -> Result<(), error::Error> {
        let mut scanner = Scanner::new("(((10) * 2.5))");
        expect_token(&mut scanner, Token::LeftParentheses);
        expect_token(&mut scanner, Token::LeftParentheses);
        expect_token(&mut scanner, Token::LeftParentheses);
        expect_token(&mut scanner, Token::Number(Number(BasicNumber::Int(10))));
        expect_token(&mut scanner, Token::RightParentheses);
        expect_token(
            &mut scanner,
            Token::BinaryOperator(BinaryOperator::Multiply),
        );
        expect_token(&mut scanner, Token::Number(Number(BasicNumber::Float(2.5))));
        expect_token(&mut scanner, Token::RightParentheses);
        expect_token(&mut scanner, Token::RightParentheses);
        assert_eq!(scanner.next(), None);
        assert!(scanner.parentheses_stack.is_empty());
        Ok(())
    }
}
