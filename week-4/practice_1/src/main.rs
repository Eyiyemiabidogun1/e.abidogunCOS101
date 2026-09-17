use std::io;

fn main() {
    println!("\nStudent Information Management System");

    // input name
    println!("\nPlease Enter your name");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Input is incorrect, Try only characters");
    println!("Your name is: {}", name);

    //input age
    println!("\nPlease enter your age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("wrong input");
    let age:u8 = age.trim().parse().expect("Input is not an integer");

    println!("Your age is: {}", age);
}