fn main() {
    if_statement();
}

// There are two types of if statements (to my understanding).
// 1. Regular if / else if / else blocks
// 2. One-line if / else if / else statements
fn if_statement(){
    let temp = 15;
// 1. Regular if / else if / else blocks
    if temp > 25 {
        println!("So hot!");
    } else if temp < 10 {
        println!("Too cold ya!");
    } else {
        println!("Nice weather!")
    }
// 2. One-line if / else if / else statements
    let day = if temp > 20 {"Sunny!"} else { "Cloudy..." };
    println!("Today is {}", day)
}

