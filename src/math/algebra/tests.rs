
#[cfg(test)]
mod tests {
	use crate::math::algebra::basic::Adder;
	use crate::math::math::{Constant, Function, Value};
	use crate::math::num::Integer;

	fn test_1() {
		let adder = Adder::<Integer>::new(vec![Constant(Integer(1)), Constant(Integer(5))]);
		assert_eq!(adder.arg_count(), 2);
		assert_eq!(adder.get_arg(0), Integer(1));
		assert_eq!(adder.get_arg(1), Integer(5));
		assert_eq!(adder.calculate(), Integer(6));
		assert!(adder.is_constant_to("any_var"));
	}
}