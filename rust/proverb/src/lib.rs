pub fn build_proverb(list: &[&str]) -> String {
	if list.is_empty() {
		return String::new();
	}

	list.windows(2).fold(String::new(), |acc, e| {
		format!("{acc}For want of a {} the {} was lost.\n", e[0], e[1])
	}) + &format!("And all for the want of a {}.", list[0])
}
