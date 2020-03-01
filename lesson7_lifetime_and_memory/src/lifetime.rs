// struct Person {
//     name: String
// }

// struct Company<'z> {
//     name: String,
//     // While the company exists, we have to ensure Rust that the Person will exist too
//     ceo: &'z Person
// }

// pub fn enter() {
//     // 'static' means the variable is going to live as long as the program lives
//     // &'static str
//     let boss = Person { name: String::from("Elon Musk") };
//     let tesla = Company { name: String::from("Tesla"), ceo: &boss };
// }

// Second Exmaple

struct Person {
    name: String
}

impl Person {
    // The two following definitions are equal
    // fn get_ref_name(&self) -> &String {
    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

struct Company<'z> {
    name: String,
    // While the company exists, we have to ensure Rust that the Person will exist too
    ceo: &'z Person
}

pub fn enter() {
    let mut z: &String;
    {
        let p = Person {name: String::from("John")};
        z = p.get_ref_name();
    }
}