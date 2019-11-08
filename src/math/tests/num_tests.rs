
mod basic_number {
	use crate::math::basic_number::BasicNumber;
	#[test]
	fn construction() {
		let f1: BasicNumber = 1f64.into();
		assert_eq!(f1, BasicNumber::Float(1f64));

		let i1: BasicNumber = 1i64.into();
		assert_eq!(i1, BasicNumber::Int(1i64));
	}
}