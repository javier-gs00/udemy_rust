struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

pub fn enter() {
    let mut clever: Creature;
    {
        let _goblin = Creature::new("Jeff");
        println!("game proceeds");
        // drop(_goblin) // manually drop the creature
        clever = _goblin;
        println!("end of scope");
    }
}