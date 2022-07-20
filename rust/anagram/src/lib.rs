use std::collections::{HashMap, HashSet};

trait Anagram: Sized {
    fn is_anagram_of(self, other: Self) -> bool;
    fn has_anagram(self, other: Self) -> bool {
        other.is_anagram_of(self)
    }
}

impl Anagram for &str {
    fn is_anagram_of(self, other: &str) -> bool {
        let lhs = self.to_lowercase();
        let rhs = other.to_lowercase();

        if lhs == rhs {
            return false;
        }

        let mut counts = HashMap::new();

        for char in lhs.chars() {
            let count = counts.entry(char).or_insert(0);
            *count += 1;
        }

        for char in rhs.chars() {
            let count = counts.entry(char).or_insert(0);
            *count -= 1;
        }

        for count in counts.values() {
            if *count != 0 {
                return false;
            }
        }

        true
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res = HashSet::new();

    for &pa in possible_anagrams {
        if pa.is_anagram_of(word) {
            res.insert(pa);
        }
    }

    res
}
