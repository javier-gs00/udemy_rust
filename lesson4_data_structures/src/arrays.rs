use std::mem;

pub fn arrays() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    print!("a has {} elements, first is {}\n", a.len(), a[0]);

    a[0] = 321;

    print!("a[0] = {}\n", a[0]);

    print!("{:?}\n", a);

    if a != [1, 2, 3, 4, 5] {
        // array length must match
        print!("does not match\n");
    }

    let b = [1u16; 10];

    for i in 0..b.len() {
        print!("{}\n", b[i]);
    }

    print!("b took up {} bytes", mem::size_of_val(&b));

    // multidimensional arrays

    // 2 rows, 3 columns
    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];

    print!("{:?}\n", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                print!("mtx[{}][{}] = {}\n", i, j, mtx[i][j]);
            }
        }
    }

    print!("\n");
}
