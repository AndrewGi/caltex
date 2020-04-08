use core::str::FromStr;
use core::num::ParseFloatError;
use core::ops::Neg;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum BasicNumber {
	Float(f64),
	Int(i64),
}
impl BasicNumber {
	pub fn as_f64(self) -> f64 {
		match self {
			BasicNumber::Float(f) => f,
			BasicNumber::Int(i) => i as f64,
		}
	}
}
impl Default for BasicNumber {
	fn default() -> Self {
		BasicNumber::Int(0)
	}
}
impl FromStr for BasicNumber {
	type Err = ParseFloatError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.parse::<i64>() {
			Ok(int) => Ok(BasicNumber::Int(int)),
			Err(_) => {
				Ok(BasicNumber::Float(s.parse::<f64>()?))
			}
		}
	}
}
impl std::ops::Add for BasicNumber {
	type Output = BasicNumber;

	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(BasicNumber::Int(a), BasicNumber::Int(b)) => BasicNumber::Int(a+b),
			_ => BasicNumber::Float(self.as_f64() + rhs.as_f64())
		}
	}
}

impl std::ops::Sub for BasicNumber {
	type Output = BasicNumber;

	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(BasicNumber::Int(a), BasicNumber::Int(b)) => BasicNumber::Int(a - b),
			_ => BasicNumber::Float(self.as_f64() - rhs.as_f64())
		}
	}
}

impl std::ops::Mul for BasicNumber {
	type Output = BasicNumber;

	fn mul(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(BasicNumber::Int(a), BasicNumber::Int(b)) => BasicNumber::Int(a*b),
			_ => BasicNumber::Float(self.as_f64() * rhs.as_f64())
		}
	}
}

impl std::ops::Div for BasicNumber {
	type Output = BasicNumber;

	fn div(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(BasicNumber::Int(a), BasicNumber::Int(b)) => BasicNumber::Int(a/b),
			_ => BasicNumber::Float(self.as_f64() / rhs.as_f64())
		}
	}
}
impl std::ops::Neg for BasicNumber {
	type Output = BasicNumber;

	fn neg(self) -> Self::Output {
		match self {
			BasicNumber::Int(i) => BasicNumber::Int(-i),
			BasicNumber::Float(f) => BasicNumber::Float(-f),
		}
	}
}