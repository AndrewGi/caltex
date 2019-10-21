
impl Default for Number {
	fn default() -> Self {
		Self {n: 0f64}
	}
}
impl std::ops::Add for Number {
	type Output = Number;

	fn add(self, rhs: Self) -> Self::Output {
		Self::Output(self.n + rhs.n)
	}
}
impl std::ops::Sub for Number {
	type Output = Number;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::Output(self.n - rhs.n)
	}
}

impl std::ops::Mul for Number {
	type Output = Number;

	fn mul(self, rhs: Self) -> Self::Output {
		Self::Output(self.n * rhs.n)
	}
}

impl std::ops::Div for Number {
	type Output = Number;

	fn div(self, rhs: Self) -> Self::Output {
		Self::Output(self.n / rhs.n)
	}
}
pub trait Value {
	fn get_number(&self) -> Number;
}

pub trait Function: Value {
	fn arg_count(&self) -> usize;
	fn get_arg(&self, arg_i: usize) -> &dyn Value;
}