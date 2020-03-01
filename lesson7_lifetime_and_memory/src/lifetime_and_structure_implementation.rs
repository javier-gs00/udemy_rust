struct Person<'a> {
    name: &'a str
}

impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi, my name is {}", self.name)
    }
}

pub fn enter() {
    let person = Person { name: "Dmitri" };
    person.talk();
}