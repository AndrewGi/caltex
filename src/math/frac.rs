use crate::math::math::{Value, GCD};
use crate::math::num;
use std::ops::{Add, Mul, Div};
use crate::math::num::{Number, One, Zero};

#[derive(Copy, Clone, Eq)]
pub struct Frac<Num: Number + GCD> {
	numerator: Num,
	denominator: Num,
}
impl<Num: Number> From<Num> for Frac<Num> {
    fn from(n: Num) -> Self {
        Frac::new(n, Frac::one())
    }
}
impl<Num: Number + GCD> Frac<Num> {
	pub fn new(numerator: Num, denominator: Num) -> Frac<Num> {
		Frac {
			numerator,
			denominator
		}
	}
	pub fn invert(self) -> Frac<Num> {
		Frac::new(self.denominator, self.numerator)
	}
	pub fn simplify(&self) -> Frac<Num>{
		let gcd = self.numerator.gcd(self.denominator);
		Frac::new(self.numerator/gcd, self.denominator/gcd)
	}
}
impl<Num: Number> Zero for Frac<Num> {
	fn zero() -> Self {
		Frac::new(Num::zero(), Num::one())
	}
	fn is_zero(&self) -> bool {
		self.numerator.is_zero()
	}
}
impl<Num: Number> One for Frac<Num> {
	fn one() -> Self {
		Frac::new(Num::one(), Num::one())
	}
}
impl<Num: Number> Default for Frac<Num> {
	fn default() -> Self {
        Frac::one()
	}
}
impl<Num: Number + GCD> Add for Frac<Num> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.numerator *
    }
}
impl<Num: Number + GCD> Mul for Frac<Num> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Frac::new(self.numerator * rhs.numerator, self.denominator * rhs.denominator).simplify()
	}
}
impl<Num: Number> Div for Frac<Num> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		self * rhs.invert()
	}
}