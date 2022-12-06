pub fn raindrops(n: u32) -> String {
	match [(3, "Pling"), (5, "Plang"), (7, "Plong")]
		.iter()
		.fold(String::new(), |acc, (k, v)| match n % k {
			0 => acc + v,
			_ => acc,
		}) {
		x if x == "" => format!("{n}"),
		x => x,
	}
}
