use std::io;
fn add(a :i32, b:i32) {
    let sum = a+b;

    println!("The sum of A and B is = {}", sum);
}
fn main(){
    let mut input = String::new();
    println!("Enter the parameter for A: ");
    io::stdin().read_line(&mut input).expect("Failed to register");
    let a:i32 = input.trim().parse().expect("input and integer");

    let mut input2 = String::new();
    println!("Enter the parameter for B: ");
    io::stdin().read_line(&mut input2).expect("invalid input");
    let b:i32 = input2.trim().parse().expect("input and integer");
    
    add(a,b);
}