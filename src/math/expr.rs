use crate::math::math::{Value, MathError};
use crate::math::num::Number;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub struct BinaryExpression<Num: Number> {
    left: Box<dyn Value<Num>>,
    right: Box<dyn Value<Num>>,
    operator: BinaryOperator
}
impl<Num: Number> Value<Num> for BinaryExpression<Num> {
    fn calculate(&self) -> Result<Num, MathError> {
        let left_num = self.left.calculate()?;
        let right_num = self.right.calculate()?;
        let result = match self.operator {
            BinaryOperator::Addition => left_num + right_num,
            BinaryOperator::Subtraction => left_num - right_num,
            BinaryOperator::Division => left_num / right_num,
            BinaryOperator::Multiplication => left_num * right_num,
        };
        Ok(result)
    }
    fn is_constant(&self) -> bool {
        self.left.is_constant() && self.right.is_constant()
    }
    fn is_constant_to(&self, variable_name: &str) -> bool {
        self.left.is_constant_to(variable_name) && self.right.is_constant_to(variable_name)
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Absolute,
}
pub struct UnaryOperation<Num: Number> {
    operator: UnaryOperator,
    operand: Box<dyn Value<Num>>
}
impl<Num: Number> Value<Num> for UnaryOperation<Num> {
    fn calculate(&self) -> Result<Num, MathError> {
        let v = self.operand.calculate()?;
        match self.operator {
            UnaryOperator::Negate => Ok(v.neg()),

        }
    }

    fn is_constant(&self) -> bool {
        unimplemented!()
    }

    fn is_constant_to(&self, variable_name: &str) -> bool {
        unimplemented!()
    }
}