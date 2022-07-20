pub fn annotate(minefield: &[&str]) -> Vec<String> {
	#[rustfmt::skip]
	let adjacent = |r, c| [
		(r-1,c-1), (r-1,c), (r-1,c+1),
		(r,  c-1),          (r,  c+1),
		(r+1,c-1), (r+1,c), (r+1,c+1)
	];

	let w = minefield.len();
	let h = if w > 0 { minefield[0].len() } else { 0 };

	minefield
		.iter()
		.enumerate()
		.map(|(i, row)| {
			row.bytes()
				.enumerate()
				.map(|(j, col)| match col {
					b'*' => '*',
					_ => match adjacent(i as i8, j as i8)
						.iter()
						.filter(|(r, c)| {
							(0..w).contains(&(*r as usize)) && (0..h).contains(&(*c as usize))
						})
						.filter(|(r, c)| minefield[*r as usize].as_bytes()[*c as usize] == b'*')
						.count()
					{
						0 => ' ',
						c => (c as u8 + b'0') as char,
					},
				})
				.collect()
		})
		.collect()
}
