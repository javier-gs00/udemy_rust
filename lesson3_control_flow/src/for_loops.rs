pub fn for_loops() {
    for x in 1..11 { // from 1 to 10, break and continue statements also work
        if x == 3 { continue; }

        if x == 8 { break; }

        println!("x = {}", x);
    }

    // .enumerate is needed for getting the current position
    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}