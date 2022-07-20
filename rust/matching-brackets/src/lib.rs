use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
	let open = HashMap::from([('}', '{'), (']', '['), (')', '(')]);
	let mut stack: Vec<char> = vec![];

	for char in string.chars() {
		match char {
			'{' | '[' | '(' => stack.push(char),
			'}' | ']' | ')' => {
				if stack.pop() != Some(open[&char]) {
					return false;
				}
			}
			_ => continue,
		}
	}

	stack.len() == 0
}
