use Direction::{East, North, South, West};

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
	North,
	East,
	South,
	West,
}

impl Direction {
	fn by_discriminant(d: usize) -> Self {
		match d {
			0 => North,
			1 => East,
			2 => South,
			3 => West,
			_ => panic!(),
		}
	}
}

pub struct Robot {
	x: i32,
	y: i32,
	d: Direction,
}

impl Robot {
	pub fn new(x: i32, y: i32, d: Direction) -> Self {
		Robot { x, y, d }
	}

	#[must_use]
	pub fn turn_right(self) -> Self {
		Robot {
			d: Direction::by_discriminant((self.d as usize + 1) % 4),
			..self
		}
	}

	#[must_use]
	pub fn turn_left(self) -> Self {
		Robot {
			d: Direction::by_discriminant((self.d as usize + 3) % 4),
			..self
		}
	}

	#[must_use]
	pub fn advance(self) -> Self {
		match self.d {
			North => Robot {
				y: self.y + 1,
				..self
			},
			East => Robot {
				x: self.x + 1,
				..self
			},
			South => Robot {
				y: self.y - 1,
				..self
			},
			West => Robot {
				x: self.x - 1,
				..self
			},
		}
	}

	#[must_use]
	pub fn instructions(self, instructions: &str) -> Self {
		let mut bot = self;

		for ins in instructions.chars() {
			bot = match ins {
				'R' => bot.turn_right(),
				'L' => bot.turn_left(),
				'A' => bot.advance(),
				_ => panic!("Instructions evil!"),
			}
		}

		return bot;
	}

	pub fn position(&self) -> (i32, i32) {
		(self.x, self.y)
	}

	pub fn direction(&self) -> &Direction {
		&self.d
	}
}
