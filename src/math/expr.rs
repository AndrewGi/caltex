use crate::math::math::Value;
use crate::math::num::Number;

pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,


}

pub struct BinaryExpression {
    left: Box<dyn Value>,
    right: Box<dyn Value>,
    operator: BinaryOperator
}
impl Value for BinaryExpression {
    fn calculate(&self) -> Self::Output {
        let left_num = self.left.calculate();
        let right_num = self.right.calculate();
        let result = match self.operator {
            BinaryOperator::Addition => left_num + right_num,
            BinaryOperator::Subtraction => left_num - right_num,
            BinaryOperator::Division => left_num / right_num,
            BinaryOperator::Multiplication => left_num * right_num,
        };
        result
    }
}