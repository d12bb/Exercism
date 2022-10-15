pub fn verse(n: u32) -> String {
	let p1 = "of beer on the wall,";
	let p2 = "of beer.";
	let p3 = [
		"Go to the store and buy some more, 99 bottles",
		"Take it down and pass it around, no more bottles",
		"Take one down and pass it around,",
	];
	let p4 = "of beer on the wall.\n";
	match n {
		0 => format!("No more bottles {p1} no more bottles {p2}\n{} {p4}", p3[0]),
		1 => format!("1 bottle {p1} 1 bottle {p2}\n{} {p4}", p3[1]),
		2 => format!("2 bottles {p1} 2 bottles {p2}\n{} 1 bottle {p4}", p3[2]),
		_ => format!(
			"{n} bottles {p1} {n} bottles {p2}\n{} {} bottles {p4}",
			p3[2],
			n - 1
		),
	}
}

pub fn sing(start: u32, end: u32) -> String {
	(end..=start)
		.fold(String::new(), |res, n| verse(n) + "\n" + &res)
		.trim_end()
		.to_owned()
		+ "\n"
}
