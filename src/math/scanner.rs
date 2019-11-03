use crate::math::basic_number::BasicNumber;
use std::str::Chars;
use std::num::{ParseIntError, ParseFloatError};
use std::ops::Neg;
use crate::math::scanner::ScannerError::UnexpectedCharacter;
use std::convert::TryFrom;


pub enum ScannerError<'a> {
	EOL,
	UnexpectedCharacter(char),
	ParseIntError(ParseIntError),
	ParseFloatError(ParseFloatError)
}

pub struct Number(BasicNumber);
pub enum Operator {
	Plus,
	Minus,
	Multiply,
	Divide
}
impl Operator {
	pub fn which_operator(c: char) -> Option<Operator> {
		match c {
			'+' => Some(Operator::Plus),
			'-' => Some(Operator::Minus),
			'*' => Some(Operator::Multiply),
			'/' => Some(Operator::Divide),
			_ => None,
		}
	}
}
pub struct Power<'a>(Token<'a>);
pub struct Text<'a>(&'a str);
pub struct Parentheses<'a>(Scanner<'a>);

pub enum Token<'a> {
	Number(Number),
	Operator(Operator),
	Text(Text<'a>),
	Parentheses(Parentheses<'a>),
}
#[derive(Clone)]
pub struct Scanner<'a> {
	chars: Chars<'a>
}
impl<'a> Iterator for Scanner<'a> {
	type Item = Token<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.next_token().ok()
	}
}
impl<'a> From<&'a str> for Scanner<'a> {
	fn from(string: &str) -> Self {
		Self::new(string.chars())
	}
}
impl<'a> Scanner<'a> {
	pub fn new(c: Chars<'_>) -> Scanner<'_> {
		Self {
			chars: c
		}
	}
	pub fn is_done(&self) -> bool {
		self.len() == 0
	}
	pub fn len(&self) -> usize {
		self.as_str().len()
	}
	pub fn peek_char(&self) -> Option<char> {
		self.clone().next_char()
	}
	pub fn as_str(&self) -> &str {
		self.chars.as_str()
	}
	pub fn span_to(&self, other: &Self) -> Scanner<'a> {
		let distance_in_bytes= self.as_str().as_ptr().wrapping_offset_from(other.as_str().as_ptr());
		let u_dist = match usize::try_from(distance_in_bytes) {
			Ok(i) => i,
			Err(_) => panic!("scanner self.span_to(other): other is a parent of self"),
		};
		&self.as_str()[..u_dist].into()
	}
	fn consume_if(&mut self, func: impl Fn(char) -> bool) -> Option<char> {
		if func(self.peek_char()?) {
			self.next_char()
		} else {
			None
		}
	}
	pub fn next_char(&mut self) -> Option<char> {
		self.chars.next()
	}
	pub fn next_number(&mut self) -> Result<Number, ScannerError<'a>> {
		let mut cloned = self.clone();
		let is_neg = cloned.consume_if(|c| c=='-').is_some();
		while cloned.consume_if(char::is_numeric).is_some() {}
		let num = if cloned.consume_if(|c|c=='.').is_none() {
			//Its a int.
			BasicNumber::Int(cloned.span_to(self).ok_or(ScannerError::EOL)?.parse().map_err(|e| ScannerError::ParseIntError(e))?)
		} else {
			//Looks like a float.
			while cloned.consume_if(char::is_numeric).is_some() {}
			BasicNumber::Float(cloned.span_to(self).ok_or(ScannerError::EOL)?.parse().map_err(|e| ScannerError::ParseFloatError(e))?)
		};
		*self = cloned;
		Ok(Number(if is_neg {
			num.neg()
		} else {
			num
		}))
	}
	pub fn next_text(&mut self) -> Result<Text<'a>, ScannerError> {
		let cloned = self.clone();
		let c=  self.peek_char().ok_or(ScannerError::EOL)?;
		if !c.is_alphabetic() {
			return Err(ScannerError::UnexpectedCharacter(c));
		}
		while self.consume_if(char::is_alphabetic).is_some() {};
		Ok(Text(cloned.span_to(self).unwrap()))
	}
	pub fn consume_whitespace(&mut self) {
		while self.consume_if(char::is_whitespace) {}
	}
	pub fn next_operator(&mut self) -> Result<Operator, ScannerError>{
		let c= self.peek_char().ok_or(ScannerError::EOL)?;
		if let Some(op) = Operator::which_operator(c) {
			self.next_char().ok_or(ScannerError::EOL)?;
			Ok(op)
		} else {
			Err(UnexpectedCharacter(c))
		}
	}
	fn next_parentheses(&mut self) -> Result<Parentheses<'a>, ScannerError<'a>> {
		let mut clone = self.clone();
		clone.consume_if(|c| c=='(').ok_or(ScannerError::EOL)?;
		let mut i = 1;
		while i > 0 {
			match clone.next_char().ok_or(ScannerError::EOL)? {
				'(' => i += 1,
				')' => i -= 1,
				_ => ()
			}
		}
		Ok(Parentheses(self.span_to(&clone)))
	}
	pub fn next_token(&mut self) -> Result<Token<'a>, ScannerError<'a>> {
		self.consume_whitespace();
		let mut clone = self.clone();
		Ok(match self.peek_char().ok_or(ScannerError::EOL)? {
			'.' => Token::Number(self.next_number()?),
			'(' => Token::Parentheses(self.next_parentheses()?),
			c if c.is_alphabetic() => self.next_text().map(|text| Token::Text(text)),
			c if c.is_digit(10) => self.next_number().map(|num| Token::Number(num)),
			'-' => {
				if self.clone().next_number().is_ok() {
					self.next_number().map(|num| Token::Number(num))
				} else {
					self.next_operator().map(|op| Token::Operator(op))
				}
			},
			c => {
				if let Some(op) = Operator::which_operator(c) {
					Token::Operator(op)
				} else {
					return Err(UnexpectedCharacter(c))
				}
			}
		})
	}
}