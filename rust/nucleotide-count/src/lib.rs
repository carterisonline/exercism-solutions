use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut all_nucleotides: bool = true;
    let mut err_char: char = nucleotide;
    dna.chars().for_each(|c| {
        if is_nucleotide(c).is_some() {
            err_char = c;
            all_nucleotides = false;
        }
    });
    if is_nucleotide(nucleotide).is_some() || !all_nucleotides {
        return Err(err_char);
    }
    Ok(dna.matches(nucleotide).collect::<Vec<&str>>().len())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    unimplemented!(
        "How much of every nucleotide type is contained inside DNA string '{}'?",
        dna
    );
}

fn is_nucleotide(ch: char) -> Option<char> {
    match ch {
        'a' | 't' | 'g' | 'c' | 'A' | 'T' | 'G' | 'C' => None,
        _ => Some(ch),
    }
}
