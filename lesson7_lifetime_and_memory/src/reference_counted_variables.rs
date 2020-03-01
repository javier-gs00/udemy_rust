use std::rc::Rc;

struct Person {
    name: Rc<String>
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person {name: name}
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

pub fn enter() {
    // Rc allows us to share this variable
    // let name = Rc::new("John".to_string());
    // let person = Person::new(name.clone());

    // person.greet();
    // println!("Name = {}", name);

    // ------------

    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
}