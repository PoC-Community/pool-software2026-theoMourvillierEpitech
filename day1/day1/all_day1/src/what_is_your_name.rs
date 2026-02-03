use std::io;
pub fn what_isyour_name() -> String {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let answer = name.trim();
    format!("Your name is {}", answer)
}