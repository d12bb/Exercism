#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
	pub fn new(dna: &str) -> Result<Dna, usize> {
		if let Some((i, _)) = dna
			.chars()
			.enumerate()
			.find(|(_, c)| !['G', 'C', 'T', 'A'].contains(c))
		{
			return Err(i);
		}
		return Ok(Dna(String::from(dna)));
	}

	pub fn into_rna(self) -> Rna {
		Rna(self
			.0
			.chars()
			.map(|c| match c {
				'G' => 'C',
				'C' => 'G',
				'T' => 'A',
				'A' => 'U',
				_ => panic!(),
			})
			.collect())
	}
}

impl Rna {
	pub fn new(rna: &str) -> Result<Rna, usize> {
		if let Some((i, _)) = rna
			.chars()
			.enumerate()
			.find(|(_, c)| !['C', 'G', 'A', 'U'].contains(c))
		{
			return Err(i);
		}
		return Ok(Rna(String::from(rna)));
	}
}
