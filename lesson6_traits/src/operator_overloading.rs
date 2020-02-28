use std::ops::{Add, AddAssign, Neg};
use std::cmp::{PartialEq};

#[derive(Debug, PartialEq, Eq)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

impl<T> Add for Complex<T>
    where T: Add<Output = T>
    {
    type Output = Complex<T>;

    // a + b
    // self is the left side (a)
    // right is the b (rhs: right hand side)
    // Self with capital S, is the type where we are right now... i32 in this case
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

//  this will allow += to work on Complex numbers
impl<T> AddAssign for Complex<T>
    where T: AddAssign<T> {
        fn add_assign(&mut self, rhs: Self) {
            self.re += rhs.re;
            self.im += rhs.im;
        }
    }

impl<T> Neg for Complex<T>
    where T: Neg<Output=T> {
        type Output = Complex<T>;

        fn neg(self) -> Self::Output {
            Complex {
                re: -self.re,
                im: -self.im
            }
        }
    }

// partial equality
// full eq: x = x
// NAN = not a number e.g 0/0 inf/inf
// NAN == NAN -> false
// impl<T> PartialEq for Complex<T>
//     where T: PartialEq {
//         fn eq(&self, rhs: &Self) -> bool {
//             self.re == rhs.re && self.im == rhs.im
//         }
//     }

// // implement full equality
// impl<T: Eq> Eq for Complex<T> where T: Eq {}

// But... PartialEq and Eq can be derived!!! see above

pub fn enter() {
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.0, 4.0);

    // println!("{:?}", a + b);
    // a += b;
    println!("{:?}", a == a);

}