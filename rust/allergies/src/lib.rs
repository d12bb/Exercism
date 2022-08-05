pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
	Eggs,
	Peanuts,
	Shellfish,
	Strawberries,
	Tomatoes,
	Chocolate,
	Pollen,
	Cats,
}

// Could use strum crate to iter over Enum variants instead of manually created array
const ALLERGENS: [Allergen; 8] = [
	Allergen::Eggs,
	Allergen::Peanuts,
	Allergen::Shellfish,
	Allergen::Strawberries,
	Allergen::Tomatoes,
	Allergen::Chocolate,
	Allergen::Pollen,
	Allergen::Cats,
];

impl Allergies {
	pub fn new(score: u32) -> Self {
		Allergies(score % 256)
	}

	pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
		(self.0 >> *allergen as u8) % 2 == 1
	}

	pub fn allergies(&self) -> Vec<Allergen> {
		ALLERGENS
			.iter()
			.filter_map(|&a| match (self.0 >> a as u8) % 2 {
				1 => Some(a),
				_ => None,
			})
			.collect()
	}
}
