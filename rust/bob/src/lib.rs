pub fn reply(message: &str) -> &str {
	let m = message.trim();
	let has_letters = m.chars().any(|c| c.is_ascii_alphabetic());
	let yelling = has_letters && !m.chars().any(|c| c.is_ascii_lowercase());
	let question = m.ends_with("?");

	match m {
		_ if question && yelling => "Calm down, I know what I'm doing!",
		_ if question => "Sure.",
		_ if yelling => "Whoa, chill out!",
		"" => "Fine. Be that way!",
		_ => "Whatever.",
	}
}
