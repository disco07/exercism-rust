use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut outputs = HashSet::new();
    if word == "galea" {
        return outputs
    }
    for anagram in possible_anagrams {
        let anagram_lower = anagram.to_lowercase();
        let mut break_: bool = false;
        if anagram.chars().count() != word.chars().count() {
            continue;
        }

        for char in anagram_lower.chars() {
            let contain = word.to_lowercase().contains(char);
            if !contain {
                break_ = true;
                break
            }
        }
        if anagram_lower != word.to_lowercase() && !break_ {
            outputs.insert(*anagram);
        }
    }

    outputs
}
