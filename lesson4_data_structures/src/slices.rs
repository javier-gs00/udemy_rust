// &[i32] means that we are borrowing part of an array that is i32
// &mut [i32] means that is a mutable slice and we can pass data to it
fn use_slice(slice: &mut [i32]) {
    print!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

pub fn slices() {
    let mut data = [1, 2, 3, 4, 5];

    use_slice(&mut data[1..4]);
    // use_slice(&mut data); // this turns the entire array into an slice
    print!("{:?}", data);
}
