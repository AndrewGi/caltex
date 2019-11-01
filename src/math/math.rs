use crate::math::num::Number;
use std::fmt::Display;

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
	fn calculate(&self) -> Option<Num>;
	fn is_constant_to(&self, variable_name: &str) -> bool;
}
pub struct Constant<Num: Number>(Num);
impl<Num: Number> Value<Num> for Constant<Num> {
	fn calculate(&self) -> Option<Num> {
		Some(self.0)
	}


	fn is_constant_to(&self, variable_name: &str) -> bool {
		true
	}
}
pub trait Function<Num: Number>: Value<Num> {

	fn arg_count(&self) -> usize;
	fn get_arg(&self, arg_i: usize) -> &dyn Value<Num>;
}
pub trait Variable<Num: Number>: Value<Num> {
	fn get_name(&self) -> &str;
	fn is_defined(&self) -> bool;
}