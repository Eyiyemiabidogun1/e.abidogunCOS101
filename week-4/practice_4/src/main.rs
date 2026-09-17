use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter Your name:");
    io::stdin().read_line(&mut input1).expect("Invalid String");

    println!("Enter a valid Number:");
    io::stdin().read_line(&mut input2).expect("Invalid String");
    let age:u8 = input2.trim().parse().expect("Invalid Number");

    if age >= 18 {
        println!("Welcome to the party: {}", input1);
    }
    else {
        println!("Oops!, you are not of age to enter the party: {}", input1);
    }
}