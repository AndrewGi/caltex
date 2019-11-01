use crate::math::math::{Value, GCD};
use crate::math::num;
use std::ops::{Add, Mul, Div};
use crate::math::num::Number;

#[derive(Clone)]
pub struct Frac<Num: Number> {
	numerator: Num,
	denominator: Num,
}

impl<Num: Number> Frac<Num> {
	pub fn new(numerator: Num, denominator: Num) -> Frac<Num> {
		Frac {
			numerator,
			denominator
		}
	}
	pub fn invert(self) -> Frac<Num> {
		Frac::new(self.denominator, self.numerator)
	}
	pub fn simplify(self) -> Frac<Num> where Num: GCD {
		let gcd = self.numerator.gcd(self.denominator);
		Frac::new(self.numerator/gcd, self.denominator/gcd)
	}
}
impl<Num: Number> Default for Frac<Num> {
	fn default() -> Self {
		1.into()
	}
}
impl<Num: Number> Mul for Frac<Num> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Frac (self.numerator * rhs.numerator, self.denominator * rhs.denominator)
	}
}
impl<Num: Number> Div for Frac<Num> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		self * rhs.invert()
	}
}