pub fn enter() {
    let v = vec![1,2,3];

    // First example

    // in this case we are copying a pointer
    // so now we have two pointers to the same memory
    // let v2 = v; // Here the value is moved from v to v2

    // this will fail because v has been moved
    // Rust internally decides to only allow v2 to access the memory
    // println!("{:?}", v);

    // Second example

    // let foo = |v:Vec<i32>| ();
    // foo(v);

    // println!("{:?}", v);

    // Third example

    // let u = 1;
    // let u2 = u; // For primitive types this works, here the value is copied
    // println!("u = {}, u2 = {}", u, u2);

    // If we want to replicate the move behavieour we have to use Box
    // let u = Box::new(1);
    // let u2 = u;

    // println!("u = {}, u2 = {}", u, u2);

    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };

    let vv = print_vector(v);
    println!("{}", vv[0]);
}