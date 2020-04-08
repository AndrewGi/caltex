use crate::math::num::Number;
use core::fmt::{Display};

pub enum MathError<'a> {
	DivideByZero,
	UndefinedVariable(&'a str),
	Other(String)
}

pub trait GCD {
	fn gcd(self, other: Self) -> Self;
}
impl GCD for i64 {
	fn gcd(self, other: Self) -> Self {
		let mut a = self;
		let mut b = other;
		while b != 0 {
			let t = b;
			b = a % b;
			a = t;
		}
		return a;
	}
}
pub trait Value<Num: Number>: Display {
	/// Returns the Numerically value of the the object or the error if it fails.
	fn calculate(&self) -> Result<Num, MathError>;
	fn is_constant(&self) -> bool;
	fn is_constant_to(&self, variable_name: &str) -> bool;
}
pub struct Constant<Num: Number>(Num);
impl<Num: Number> Display for Constant<Num> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "{}", self.0)
	}
}
impl<Num: Number> Value<Num> for Constant<Num> {
	fn calculate(&self) -> Result<Num, MathError> {
		Ok(self.0)
	}
	fn is_constant(&self) -> bool {
		true
	}

	fn is_constant_to(&self, variable_name: &str) -> bool {
		true
	}
}
pub trait Function<Num: Number>: Value<Num> {
	fn get_name(&self) -> &'static str;
	fn arg_count(&self) -> usize;
	fn get_arg(&self, arg_i: usize) -> &dyn Value<Num>;


	fn are_args_constant(&self) -> bool {
		for i in 0..self.arg_count() {
			if !self.get_arg(i).is_constant() {
				return false;
			}
		}
		true
	}
	fn are_args_constant_to(&self, variable_name: &str) -> bool {
		for i in 0..self.arg_count() {
			if !self.get_arg(i).is_constant_to(variable_name) {
				return false;
			}
		}
		true
	}
}
pub trait Variable<Num: Number>: Value<Num> {
	fn get_name(&self) -> &str;
	fn is_defined(&self) -> bool;
}