use std::io;
fn checker() {
    let mut input = String::new();
    println!("Enter your Character:");
    io::stdin().read_line(&mut input).expect("failed to register input");
    let ch:char = input.trim().parse().expect("input a Character");

    if ch >= '0' && ch <= '9' {
        println!("Character '{}' is a digit",ch);
    }else {
        println!("Character '{}' is not a digit",ch);
    }
}
fn main() {
    println!("Welcome! This program checks whether a Character variable contains a digit or not.");
    checker()
}