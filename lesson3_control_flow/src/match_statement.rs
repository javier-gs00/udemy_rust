pub fn match_statement() {
    let country_code = 999; // 1 to 999

    // like a switch statement in javascript
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        // if the value is no in the previous values but is the following range
        1...999 => "unknown", // notice the three dots, the value 999 is included
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country);
}