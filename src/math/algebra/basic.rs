use crate::math::num::Number;
use crate::math::math::{Value, Function};

pub struct Adder<Num: Number> {
	operands: Vec<dyn Value<Num>>,
}
impl<Num: Number> Value<Num> for Adder<Num> {
	fn calculate(&self) -> Num {
		self.operands.iter().fold(Num::zero(), |acc, x| acc + x.calculate())
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

pub struct Negator<Num: Number>()