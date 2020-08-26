pub fn encode(source: &str) -> String {
    let mut out: String = "".to_string();
    let mut count_str: &str = &source;
    while let Some(c) = &count_str.chars().next() {
        let count = &count_str.chars().take_while(|&next| next == *c).count();
        match &count {
            1 => out.push(*c),
            _ => out.push_str(&format!("{}{}", &count, &c)),
        }
        count_str = &count_str[count * c.len_utf8()..];
    }
    out
}

pub fn decode(source: &str) -> String {
    source
        .split(|c: char| !&c.is_digit(10))
        .map(|n_str| n_str.parse::<usize>().unwrap_or(1))
        .zip(source.matches(|c: char| !&c.is_digit(10)))
        .map(|(num, c)| c.repeat(num))
        .collect()
}