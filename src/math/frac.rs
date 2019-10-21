use crate::math::math::Value;
use crate::math::num;
use std::ops::{Add, Mul, Div};
use crate::latex::scanner::Number;

#[derive(Clone)]
pub struct Frac {
	numerator: Number,
	denominator: Number,
}

impl Frac {
	pub fn invert(self) -> Frac {
		Frac(self.denominator, self.numerator)
	}
}
impl Into<Frac> for i64 {
	fn into(self) -> Frac {
		Frac(Box::new(num::Number::Int(self)), Box::new(num::Number::Int(1i64)))
	}
}
impl Default for Frac {
	fn default() -> Self {
		1.into()
	}
}
impl Mul for Frac {
	type Output = Frac;

	fn mul(self, rhs: Self) -> Self::Output {
		Frac (self.numerator * rhs.numerator, self.denominator * rhs.denominator)
	}
}
impl Div for Frac {
	type Output = Frac;

	fn div(self, rhs: Self) -> Self::Output {
		self * rhs.invert()
	}
}