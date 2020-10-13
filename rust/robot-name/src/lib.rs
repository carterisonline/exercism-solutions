#![feature(assoc_char_funcs)]
extern crate rand;

use rand::{thread_rng, Rng};
use std::iter;

pub struct Robot(rand::rngs::ThreadRng);

impl Robot {
    pub fn new() -> Self {
        Robot(
            thread_rng()
        )
    }

    pub fn name<'a>(&mut self) -> &'a str {
        iter::repeat(())
                .map(|()| char::from_u32(self.0.gen_range(65, 90)).unwrap())
                .take(5)
                .collect::<String>().as_str()
    }

    pub fn reset_name(&mut self) {
        unimplemented!("Assign a new unique name to the robot.");
    }
}
