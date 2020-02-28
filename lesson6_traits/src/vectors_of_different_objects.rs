trait Animal {
    // static: called as Animal::create()
    // returns the type of the implementor
    // fn create(name: &'static str) -> self

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

impl Animal for Human {
    // fn create(name: &'static str) -> Human {
    //     Human { name }
    // }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("hello, my name is {}", self.name());
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    // fn create(name: &'static str) -> Cat {
    //     Cat { name }
    // }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

enum Creature {
    Human(Human),
    Cat(Cat)
}

pub fn enter() {
    // Problem: We want to have both Human and Cats in the same vector
    // First soluction: create a vector of Enums => let mut creatures: Vec<Creature> = ...
    let mut creatures = Vec::new();
    // creatures.push(Human{name:"John"}); // Since the first struct added is a Human, the vector think we are only adding Humans
    // creatures.push(Cat{name:"Fluffy"})
    creatures.push(Creature::Human(
        Human { name: "John" }
    ));
    creatures.push(Creature::Cat(
        Cat { name: "Fluffy" }
    ));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk()
        }
    }

    // Second solution: initialize a vector of Traits with Box (This works when the Structs share traits)
    // When using Box, the size is known
    let mut animals: Vec<Box<Animal>> = Vec::new();
    animals.push(Box::new(Human{name:"John"}));
    animals.push(Box::new(Cat{name:"Fluffy"}));

    for a in animals.iter() {
        a.talk();
    }
}