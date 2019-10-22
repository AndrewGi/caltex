use crate::math::num::Number;
use std::fmt::Display;

pub trait Value<Num: Number>: Display {
	fn calculate(&self) -> Num;
}
pub struct Constant<Num: Number>(Num);
impl<Num: Number> Value<Num> for Constant<Num> {
	fn calculate(&self) -> Num {
		self.0
	}
}
pub trait Function<Num: Number>: Value<Num> {
	fn arg_count(&self) -> usize;
	fn get_arg(&self, arg_i: usize) -> &dyn Value<Num>;
}