use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Minefield(Vec<Vec<char>>);

impl Display for Minefield {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            self.extract()
                .iter()
                .map(|s| format!("|{}|", s))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Minefield {
    fn from_raw(contents: &[&str]) -> Minefield {
        let mut out = vec![];
        contents.iter().for_each(|s| {
            out.push(s.chars().collect::<Vec<char>>());
        });

        Minefield(out)
    }

    fn extract(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|a| a.iter().collect::<String>())
            .collect::<Vec<String>>()
    }

    fn add_one(&mut self, x: usize, y: usize) {
        let row = self.0.get(y);
        if row.is_some() {
            let item = row.unwrap().get(x);
            if item.is_some() {
                let item_some = item.unwrap();
                if item_some != &'*' && !item_some.is_whitespace() {
                    self.0[y][x] = (item_some.to_digit(10).unwrap() + 49) as u8 as char
                } else if item_some != &'*' {
                    self.0[y][x] = '1';
                }
            }
        }
    }

    fn annotate(&mut self) {
        let arr = self.0.clone();
        for (y, a) in &mut arr.iter().enumerate() {
            for (x, _) in a.iter().enumerate() {
                self.annotate_only(x, y);
            }
        }
    }

    fn annotate_only(&mut self, x: usize, y: usize) {
        if self.0[y][x] == '*' {
            for i in 0..3 {
                for j in 0..3 {
                    // disgusting attempt at fixing "subtract with overflow" errors
                    if x as i32 + i as i32 - 1 > 0 && y as i32 + j as i32 - 1 > 0 {
                        self.add_one(x + i - 1, y + j - 1);
                    }
                }
            }
        }
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut m = Minefield::from_raw(minefield);
    m.annotate();
    m.extract()
}
