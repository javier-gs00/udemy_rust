pub fn enter() {
    // this function now receives a reference to the vector using the & symbol
    let print_vector = |x:&Vec<i32>| {
        println!("x[0] = {}", x[0]);
    };

    let v = vec![3,2,1];
    // instead of giving control to print_vector of the v variable
    // we pass a reference of v to print_vector (borrow for a while)
    print_vector(&v);
    println!("v[0] = {}", v[0]);

    // -------------------------------

    let mut a = 40;
    // let b = &mut a;
    // // The asterisk in front of the reference lets us access what the reference
    // // is actually refering to
    // *b += 2;

    // // This wont work because we cant use "a" until "b" releases it (unborrow)
    // println!("a = {}", a);
    {
        let b = &mut a;
        *b += 2;
    }
    // This will work because "b" only exist inside the closure and will release
    // "a" once the closure ends
    println!("a = {}", a);

    // ---------------------------
    let mut z = vec![3,2,1];

    for i in &z {
        println!("i = {}", i);
        // z.push(5); // this is now allowe because it will eventually be undefined
    }
}