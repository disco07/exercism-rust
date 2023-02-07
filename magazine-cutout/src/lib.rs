// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut notes: HashMap<&str, usize> = HashMap::new();

    for n in note {
        *notes.entry(n).or_insert(0) += 1;
    }

    for mag in magazine {
        let nb_word = notes.get(mag).unwrap_or(&0);
        let new_nb_word = || {
            match nb_word {
                0 => 0,
                _ => nb_word - 1
            }
        };

        notes.insert(mag, new_nb_word());
    }

    let val: Vec<_> = notes.values().collect();
    println!("{:?}", val.iter().max());
    *val.iter().max().unwrap() == &0_usize
}
