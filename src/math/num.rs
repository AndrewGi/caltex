
use std::ops::*;
use std::intrinsics::powf64;

pub trait Number: Add + Sub + Mul + Div + Neg +
	Clone + Copy +
	Zero + One {

}
pub struct Float(f64);
pub struct Integer(i64);
impl Number for Float {}
impl Number for Integer {}
pub trait Zero: Number {
	fn zero() -> Self;
}
impl Zero for Float {
	fn zero() -> Self {
		Float(0f64)
	}
}
impl Zero for Integer {
	fn zero() -> Self {
		Integer(0i64)
	}
}
pub trait One: Number {
	fn one() -> Self;
}

impl One for Float {
	fn one() -> Self {
		Float(1f64)
	}
}
impl One for Integer {
	fn one() -> Self {
		Integer(1i64)
	}
}
pub trait Trig: Number {
	fn sin(self) -> Self;
	fn cos(self) -> Self;
	fn tan(self) -> Self;
}
impl Trig for Float {
	fn sin(self) -> Self {
		Float(self.0.sin())
	}

	fn cos(self) -> Self {
		Float(self.0.cos())
	}

	fn tan(self) -> Self {
		Float(self.0.tan())
	}
}
pub trait Pow: Number {
	fn pow(self, power: Self) -> Self;
}
impl Pow for Float {
	fn pow(self, power: Self) -> Self {
		Float(self.0.powf(power.0))
	}
}
impl Pow for Integer {
	fn pow(self, power: Self) -> Self {
		Integer(self.0.pow(power.0))
	}
}
