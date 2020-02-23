pub fn if_statement() {
    let temp = 35;

    if temp > 30 {
        println!("hot outside");
    } else if temp < 10 {
        println!("is cold outside");
    } else {
        println!("temp is {}", temp);
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("day is {}", day);

    println!("is it {}", if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});

    println!("it is {}",
             if temp > 20 {
                 if temp > 30 {"very hot"} else {"hot"}
             } else if temp < 10 {"cold"} else {"OK"}
    )
}