mod ownership;
mod borrowing;
mod lifetime;
mod lifetime_and_structure_implementation;
mod reference_counted_variables;
mod atomic_reference_counted_variables;
mod mutex;

fn main() {
    // ownership::enter();
    // borrowing::enter();
    // lifetime::enter();
    // lifetime_and_structure_implementation::enter();
    // reference_counted_variables::enter();
    // atomic_reference_counted_variables::enter();
    mutex::enter();
}
