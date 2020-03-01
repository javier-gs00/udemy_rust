extern crate rand;

use rand::Rng;

pub fn enter() {
    let mut rng = rand::thread_rng();
    let b: bool = rng.gen();
}