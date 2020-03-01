pub mod greetings {
    pub mod english;
    pub mod french {
        pub fn hello() -> String { "bonjour".to_string() }
        pub fn goodbye() -> String { "au revoir".to_string() }
    }
}

// #[test]
// // #[ignore]
// fn english_greeting_correct() {
//     assert_eq!("hello", greetings::english::hello());
// }

// #[test]
// #[should_panic]
// fn english_greeting_incorrect() {
//     assert_eq!("adios", greetings::english::goodbye());
// }