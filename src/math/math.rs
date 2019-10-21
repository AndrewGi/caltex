use crate::math::num::Number;
use std::fmt::Display;

pub trait Value: Display {
	fn calculate(&self) -> Number;
}
pub struct Constant(Number);
impl Value for Constant {
	fn calculate(&self) -> Number {
		self.0
	}
}
pub trait Function: Value {
	fn arg_count(&self) -> usize;
	fn get_arg(&self, arg_i: usize) -> &dyn Value;
}