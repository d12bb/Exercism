#[derive(Debug)]
pub enum CalculatorInput {
	Add,
	Subtract,
	Multiply,
	Divide,
	Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
	use CalculatorInput::*;
	let mut stack = Vec::new();

	for x in inputs.iter() {
		match x {
			Value(val) => stack.push(*val),
			x => match (stack.pop(), stack.pop()) {
				(Some(v1), Some(v2)) => match x {
					Add => stack.push(v2 + v1),
					Subtract => stack.push(v2 - v1),
					Multiply => stack.push(v2 * v1),
					Divide => stack.push(v2 / v1),
					_ => (),
				},
				_ => return None,
			},
		}
	}

	if stack.len() == 1 {
		stack.pop()
	} else {
		None
	}
}
