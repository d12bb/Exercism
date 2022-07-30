use std::collections::HashMap;

const NUCS: [char; 4] = ['A', 'C', 'T', 'G'];
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
	if !NUCS.contains(&nucleotide) {
		return Err(nucleotide);
	}
	let mut count = 0;

	for c in dna.chars() {
		if !NUCS.contains(&c) {
			return Err(c);
		}
		if c == nucleotide {
			count += 1
		};
	}
	return Ok(count);
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
	let mut counts = HashMap::new();
	for nuc in NUCS {
		counts.insert(nuc, 0);
	}

	for nuc in dna.chars() {
		if NUCS.contains(&nuc) {
			counts.entry(nuc).and_modify(|count| *count += 1);
		} else {
			return Err(nuc);
		}
	}
	Ok(counts)
}
