// High order functions
// functions that take functions
// functions that return functions (or generartor)

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

// A function that returns a function
fn greater_than(limit: u32) 
    // this how a function is defined as a return type
    -> impl Fn(u32) -> bool
{
    // use the "move" keyword to extend the lifetime of "limit"
    move |y| y > limit
}

pub fn call_high_order_function() {
    // sum of all even squares < 500

    let limit = 500;
    let mut sum = 0;

    // let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i*i;

        // if isq > limit {
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < limit)
        .filter(|x:&u32| is_even(*x))
        .fold(0, |sum, x| sum + x);
        println!("hof sum = {}", sum2);
}