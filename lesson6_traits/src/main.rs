#![allow(dead_code)]

mod traits;
mod traits_and_parameters;
mod into;
mod drop;
mod operator_overloading;
mod static_dispatch;
mod dynamic_dispatch;
mod why_dynamic_dispatch;

fn main() {
    // traits::enter();
    // traits_and_parameters::enter();
    // into::enter();
    // drop::enter();
    // operator_overloading::enter();
    // static_dispatch::enter();
    // dynamic_dispatch::enter()
    why_dynamic_dispatch::enter()
}
