/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
	if code.trim().len() < 2 {
		return false;
	}

	let mut double = true;
	let mut invalid = false;

	let luhn = code
		.chars()
		.map(|c| c as i32 - 48)
		.filter(|d| match d {
			0..=9 => true,
			-16 => false,
			_ => { invalid = true; false }
		})
		.rev()
		.map(|d| {
			double = !double;
			if double { d * 2 } else { d }
		})
		.map(|d| if d > 9 { d - 9 } else { d })
		.sum::<i32>();

	return !invalid && luhn % 10 == 0;
}
