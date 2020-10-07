fn from_bool(b: &bool) -> usize {
    match b {
        &true => 1,
        _ => 0
    }
}

pub fn abbreviate(phrase: &str) -> String {
    if &phrase.contains(':') == &true {
        return String::from(phrase.get(0..phrase.find(':').unwrap()).unwrap())
    }

    let m: &Vec<char> = &phrase.chars().collect();

    let out: String = phrase.chars().enumerate().map(|(i, c)| {
        let prev_char: &char = m.get((i + from_bool(&(i < 1))) - 1).unwrap_or(&' ');
        if ((&prev_char == &&' ' || &prev_char == &&'-') && &c.is_alphabetic() == &true) || i < 1 || (&c.is_uppercase() == &true && &prev_char.is_uppercase() == &false) {
            c.to_uppercase().to_string().chars().nth(0).unwrap()
        }
        else {
            ' '
        }
    }).collect();
    out.replace(" ", "")
}
