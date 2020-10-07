use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let v: Vec<bool> = possible_anagrams
        .into_iter()
        .map(|s| {
            &s.len() == &word.len()
                && &s.to_uppercase() != &word.to_uppercase()
                && &word
                    .chars()
                    .map(|c| char_instances(s, &c) == char_instances(word, &c))
                    .collect::<Vec<bool>>()
                    .contains(&false)
                    == &false
        })
        .collect();
    possible_anagrams
        .into_iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if &v.get(i).unwrap() == &&true {
                return Some(*s);
            }
            None
        })
        .collect::<HashSet<&'a str>>()
}

fn char_instances<'b>(s: &'b str, c: &'b char) -> usize {
    s.to_uppercase()
        .matches(&c.to_uppercase().to_string())
        .collect::<Vec<&str>>()
        .len()
}