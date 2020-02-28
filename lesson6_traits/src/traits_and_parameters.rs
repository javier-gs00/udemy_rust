use std::fmt::Debug;

#[derive(Debug)]
struct Circle {
    radius: f64
}

#[derive(Debug)]
struct Square {
    side: f64
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// First way of specifying parameters as traits
// fn print_info(shape: impl Shape) {
//     println!("The area is {}", shape.area())
// }
// fn print_info(shape: impl Shape + Debug) {
//     println!("{:?}", shape);
//     println!("The area is {}", shape.area())
// }

// Second way of specifying parameters as traits
// This is a more concise syntax
// fn print_info<T: Shape + Debug>(shape1: T, shape2: T) {
//     println!("{:?}", shape);
//     println!("The area is {}", shape.area())
// }

// Thirs way
fn print_info<T>(shape: T)
    where T: Shape + Debug {
        println!("{:?}", shape);
        println!("The area is {}", shape.area())
    }

pub fn enter() {
    let c = Circle { radius: 2.0 };
    print_info(c);
}