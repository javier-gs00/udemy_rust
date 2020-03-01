extern crate phrases;

use phrases::greetings::french::{hello, goodbye};

pub fn enter() {
    println!("English: {}, {}",
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye(),
    );

    println!("French: {}, {}", hello(), goodbye());
}