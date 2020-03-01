use std::thread;
use std::sync::Arc;

struct Person {
    name: Arc<String>
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person {name: name}
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

pub fn enter() {
    // Rc is not a safe way to pass variables between threads
    let name = Arc::new("John".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}", name);

    t.join().unwrap();
}