

mod number {
	use super::super::super::scanner::*;
	use crate::math::basic_number::BasicNumber;

	#[test]
	fn float1() -> Result<(), ScannerError> {
		assert_eq!(Scanner::from("1.0").next_number()?, Scanner::Number(BasicNumber::Float(1f64)));
		Ok(())
	}
}