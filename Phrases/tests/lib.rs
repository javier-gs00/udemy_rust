#[cfg(test)]
mod tests {
    extern crate phrases;

    #[test]
    // #[ignore]
    fn english_greeting_correct() {
        assert_eq!("hello", phrases::greetings::english::hello());
    }
    
    #[test]
    #[should_panic]
    fn english_greeting_incorrect() {
        assert_eq!("adios", phrases::greetings::english::goodbye());
    }
}
