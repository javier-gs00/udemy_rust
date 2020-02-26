// a closure is function defined inline

fn say_hello() {
    println!("hello!");
}

pub fn call_closures() {
    let sh = say_hello; // store the function in a variable
    sh();

    // closure here
    // notice the vartical bars used to delimit the parameters
    let plus_one = |x:i32| -> i32 { x+1 };
    let x = plus_one(1);
    println!("x = {}", x);

    let two = 2;
    {
        // the compiler is infering the type in this case
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }
    // let borrow_two = &mut two;

    // T: by value
    // T&: by reference
    // &mut by mutable reference
    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}