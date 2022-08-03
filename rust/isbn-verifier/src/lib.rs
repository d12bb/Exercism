/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
	let mut multiplier = 11;

	if isbn
		.chars()
		.any(|c| !c.is_ascii_digit() && c != '-' && c != 'X')
	{
		return false;
	}

	if isbn
		.chars()
		.filter(|&c| c.is_ascii_digit() || c == 'X')
		.count() != 10
	{
		return false;
	}

	return isbn
		.chars()
		.filter_map(|c| match c {
			'X' => Some(10),
			_ => {
				if let Some(d) = c.to_digit(10) {
					multiplier -= 1;
					Some(d * multiplier)
				} else {
					None
				}
			}
		})
		.sum::<u32>()
		% 11 == 0;

	// 	for c in isbn.chars() {
	// 		if multiplier < 1 {
	// 			return false;
	// 		}
	// 		match c {
	// 			'-' => continue,
	// 			'X' | 'x' => {
	// 				sum += 10;
	// 				multiplier -= 1;
	// 			}
	// 			// digit if let Some(digit) = c.to_digit(10) => { sweet sweet nightly :D
	// 			_ => {
	// 				if let Some(digit) = c.to_digit(10) {
	// 					sum += multiplier * digit;
	// 					multiplier -= 1;
	// 				} else {
	// 					return false;
	// 				}
	// 			}
	// 		}
	// 	}
	// 	return multiplier == 0 && sum % 11 == 0;
}
