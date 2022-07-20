pub fn abbreviate(phrase: &str) -> String {
	if phrase.is_empty() {
		return "".to_string();
	}

	let mut abbr = String::new();
	let words = phrase.split([' ', '-']).filter(|w| !w.is_empty());

	for word in words {
		let uppercased: Vec<char> = word.chars().filter(|c| c.is_uppercase()).collect();
		let word = word
			.chars()
			.filter(|c| c.is_alphabetic())
			.collect::<Vec<char>>();

		if uppercased.len() > 1 && uppercased.len() < word.len() {
			// CamelCase
			for c in uppercased {
				abbr.push(c);
			}
		} else {
			abbr.push(word[0].to_ascii_uppercase());
		}
	}

	abbr
}
