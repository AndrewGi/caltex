use crate::math::num::Number;
use crate::math::math::{Value, Function, MathError};

pub struct Adder<Num: Number> {
	operands: Vec<dyn Value<Num>>,
}
impl<Num: Number> Adder<Num> {
	pub fn new(operands: Vec<dyn Value<Num>>) -> Adder<Num> {
		Adder { operands }
	}
}
impl<Num: Number> Value<Num> for Adder<Num> {
	fn calculate(&self) -> Result<Num, MathError> {
		let acc = Num::zero();
		for operand in &self.operands {
			acc += operand.calculate()?;
		}
		self.operands.iter().fold(Num::zero(), |acc, x| acc + x.calculate())
	}

	fn is_constant_to(&self, variable_name: &str) -> bool {
		for operand in &self.operands {
			if !operand.is_constant_to(variable_name) {
				return false;
			}
		}
		true
	}
}
impl<Num: Number> Function<Num> for Adder<Num> {
	fn arg_count(&self) -> usize {
		self.operands.len()
	}

	fn get_arg(&self, arg_i: usize) -> &dyn Value<Num> {
		if arg_i > self.arg_count() {
			panic!("arg_i '{}' > arg count '{}'", arg_i, self.arg_count());
		}
		&self.operands[arg_i]
	}
}

pub struct Negator<Num: Number>(Box<dyn Value<Num>>);
impl<Num: Number> Value<Num> for Negator<Num> {
	fn calculate(&self) -> Option<Num> {
		self.0.calculate().map(Num::neg)
	}

	fn is_constant_to(&self, variable_name: &str) -> bool {
		self.0.is_constant_to(variable_name)
	}
}

pub struct Inverter<Num: Number>(Box<dyn Value<Num>>);
impl<Num: Number> Value<Num> for Inverter<Num> {
	fn calculate(&self) -> Result<Num, MathError> {
		let value = self.0.calculate()?;
		if value.is_zero() {
			MathError::DivideByZero
		} else {
			Num::one() / value
		}
	}

	fn is_constant_to(&self, variable_name: &str) -> bool {
		self.0.is_constant_to(variable_name)
	}
}
