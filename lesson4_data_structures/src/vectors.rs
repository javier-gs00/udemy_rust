pub fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    print!("a = {:?}\n", a);
    a.push(44);
    print!("a = {:?}\n", a);

    // usize isize
    // the type of the index has to be unsigned
    let idx: usize = 3;
    a[idx] = 321;
    print!("a[{}] = {}\n", idx, a[idx]);

    // returns an Option type that can be matched against
    match a.get(idx) {
        Some(x) => print!("a[6] = {}\n", x),
        None => print!("error, no such element\n"),
    }

    for x in &a {
        print!("{}\n", x);
    }

    a.push(77);
    print!("{:?}\n", a);

    let last_elem = a.pop(); // returns an Option
    print!("last_elem is {:?}, a = {:?}\n", last_elem, a);

    while let Some(x) = a.pop() {
        print!("{}\n", x);
    }
}
