use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magmap: HashMap<&str, u8> = HashMap::with_capacity(magazine.len());
    let mut notemap: HashMap<&str, u8> = HashMap::with_capacity(note.len());

    for word in magazine.iter() {
        let count = magmap.entry(word).or_insert(0);
        *count += 1;
    }
    for word in note.iter() {
        let count = notemap.entry(word).or_insert(0);
        *count += 1;
    }

    for (word, count) in &notemap {
        let magcount = magmap.entry(word).or_insert(0);
        if count > magcount {
            return false;
        }
    }

    true
}
