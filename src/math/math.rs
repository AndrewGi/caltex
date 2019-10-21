
pub trait Value<Num: Number> {
	fn get_number(&self) -> Number;
}

pub trait Function: Value {
	fn arg_count(&self) -> usize;
	fn get_arg(&self, arg_i: usize) -> &dyn Value;
}