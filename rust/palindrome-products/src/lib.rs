/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
	/// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
	pub fn new(value: u64) -> Option<Palindrome> {
		if Self::check(value) {
			Some(Palindrome(value))
		} else {
			None
		}
	}

	/// Check if a number is a palindrome.
	fn check(value: u64) -> bool {
		let (mut rev, mut tmp) = (0, value);

		while tmp > 0 {
			rev = rev * 10 + tmp % 10;
			tmp /= 10;
		}

		return value == rev;
	}

	/// Get the value of this palindrome.
	pub fn into_inner(self) -> u64 {
		self.0
	}
}

pub fn palindrome_products(lower: u64, upper: u64) -> Option<(Palindrome, Palindrome)> {
	let (mut min, mut max) = (u64::MAX, u64::MIN);

	for i in lower..=upper {
		for j in i..=upper {
			let prod = i * j;
			if Palindrome::check(prod) {
				min = prod.min(min);
				max = prod.max(max);
			}
		}
	}

	return if max < min {
		None
	} else {
		Some((Palindrome::new(min).unwrap(), Palindrome::new(max).unwrap()))
	};
}
