pub fn series(digits: &str, len: usize) -> Vec<String> {
	match len {
		0 => vec![String::new(); digits.len() + 1], // ???
		_ => digits
			.chars()
			.collect::<Vec<char>>()
			.windows(len)
			.map(String::from_iter)
			.collect(),
	}
}
