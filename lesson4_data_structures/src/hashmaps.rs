use std::collections::HashMap;

pub fn entry() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    // println!("a square has {} sides", shapes["square".into()]);

    // for (key, value) in &shapes {
    //     println!("{} : {}", key, value);
    // }

    // shapes.insert("square".into(), 5);

    // insert the value if its not there
    // if it is there, modify it
    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);
    {
        let actual = shapes.entry("circle".into()).or_default();
        *actual = 0;
    }
    println!("{:?}", shapes);
}
