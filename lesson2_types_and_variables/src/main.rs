#![allow(dead_code)]

use std::mem;

fn scope_and_shadowing() {
    let a = 123;

    // let a = 777; // redeclaring a variable overrides the previous value
    // curly braces create an scope
    {
        let b = 456;
        println!("inside, b = {}", b);
        let a = 777; // this only affects the scope in which is declared in
        println!("inside, a = {}", a);
    }

    println!("a = {}", a);
}

fn operators() {
    // arithmetic
    let mut a = 2+3*4;
    println!("a = {}", a);
    a = a + 1; // rust does not support ++ --
    a -= 2; // a = a - 2
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR, & AND, ^ XOR, ! NOR
                        // 01 or 10 = 11 == 3_10
    println!("1 | 2 = {}", c);

    let two_to_10 = 1 << 10; // shift the coma ten places to the left
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    let x = 5;
    let x_is_5 = x == 5; // true
}

fn fundamental_data_types() {
    // numbers can be signed and unsigned
    // unsigned 0 +, u stands for unsigned. Possible values are 0..255
    let a:u8 = 123; // 8bits
    println!("a = {}", a);

    // mutable variable
    let mut b:i8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; // 32-bit signed i32. i stands for signed
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    // i8 u8 i16 u16 i32 u32 i64 u62
    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double-precision, 8 bytes or 64 bits, f64
    // type f32 could be used to store this float number
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // boolean
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    let f = 4 > 0; // true
}

fn main() {
    scope_and_shadowing();
}
