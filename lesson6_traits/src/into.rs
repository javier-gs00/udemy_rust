use std::fmt::Debug;

#[derive(Debug)]
struct Person {
    name: String
}

impl Person {
    // fn new(name: &str) -> Person {
    //     Person { name: name.to_string() }
    // }

    // Specify a generic type S and the specify the trait that the generic has to implement
    // fn new<S: Into<String>>(name: S) -> Person {
    //     Person { name: name.into() }
    // }

    fn new<S>(name: S) -> Person
        where S: Into<String> + Debug {
            Person { name: name.into() }
        }
}

pub fn enter() {
    let john = Person::new("John");

    let name: String = "Jane".to_string();
    let jane = Person::new(name);
    println!("john = {:?}, jane = {:?}", john, jane)
}