use std::io;

fn checker() {
    let mut ch = String::new();

    println!("Enter a character:");
    io::stdin().read_line(&mut ch).expect("Failed to read input");

    let ch = ch.trim().chars().next().unwrap();

    if ch >= '0' && ch <= '9' {
        println!("Character '{}' is a digit", ch);
    } else {
        println!("Character '{}' is not a digit", ch);
    }
}

fn main() {
    checker();
}