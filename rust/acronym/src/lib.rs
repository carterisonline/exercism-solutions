pub fn abbreviate(phrase: &str) -> String {
    if &phrase.contains(':') == &true {
        return String::from(phrase.get(0..phrase.find(':').unwrap()).unwrap())
    }

    let m: &Vec<char> = &phrase.chars().collect();

    phrase.chars().enumerate().map(|(i, c)| {
        let mut i2 = i as i32;
        if i < 1 {
            i2 = 1;
        }
        
        if &c.is_uppercase() == &true && !m[(i2 - 1) as usize].is_uppercase() {
            c
        }
        else {
            ' '
        }
    }).collect()
}
