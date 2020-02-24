pub fn strings() {
    // 1. First type of string "str"

    // the type of "s" is a reference to the static type str
    // static means that it will be included in the program memory(?)
    // &str = string slice
    // utf-8
    let s: &'static str = "hello there!";
    // s = "asd"; // cannot reassign
    // let h = s[0]; // cannot get characters inside

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first char is {}", first_char);
    }

    // 2. Second type of string "String"
    // String is in the heap memory
    // This string can be modified
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    // Conversion &str <> String
    // let u: &str = &letters;

    // Concatenation
    // Str + str
    // let z = letters + "abc";

    // let mut abc = String::from("hello world");
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}
