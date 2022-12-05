#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	NotEnoughPinsLeft,
	GameComplete,
}

pub struct BowlingGame {
	rolls: Vec<u16>,
}

impl BowlingGame {
	pub fn new() -> Self {
		BowlingGame { rolls: vec![] }
	}

	pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
		let rolls = self.rolls.len();
		let last = self.rolls.last().unwrap_or(&0);

		if self.complete() {
			return Err(Error::GameComplete);
		} else if pins > 10 {
			return Err(Error::NotEnoughPinsLeft);
		} else if rolls < 19 && rolls % 2 == 1 && last + pins > 10 {
			// >10 in one frame
			return Err(Error::NotEnoughPinsLeft);
		} else if rolls == 20 && last + self.rolls[18] != 10 && last != &10 && last + pins > 10 {
			// no spare/strike, but fill ball >10
			return Err(Error::NotEnoughPinsLeft);
		}

		self.rolls.push(pins);
		if pins == 10 && rolls < 18 && rolls % 2 == 0 {
			self.rolls.push(0);
		}
		Ok(())
	}

	pub fn score(&self) -> Option<u16> {
		if !self.complete() {
			return None;
		}

		Some(
			self.rolls
				.iter()
				.enumerate()
				.rev()
				.fold((0, 0, 0, 0), |acc, e| match (acc, e) {
					((_, _, _, _), (20, &r)) => (r, 0, 0, 0), // fill ball
					((score, _, p2, p3), (i, 10)) if i < 18 => (score + 10 + p2 + p3, 10, p2, p3), //strike
					((score, p1, p2, _), (i, &r)) if i % 2 == 0 && r + p1 == 10 => {
						(score + r + p2, r, p1, p2)
					} // spare
					((score, p1, p2, _), (_, &r)) => (score + r, r, p1, p2), // regular
				})
				.0,
		)
	}

	fn complete(&self) -> bool {
		self.rolls.len() == 21 || self.rolls.len() == 20 && self.rolls[18] + self.rolls[19] < 10
	}
}

impl Default for BowlingGame {
	fn default() -> Self {
		Self::new()
	}
}
