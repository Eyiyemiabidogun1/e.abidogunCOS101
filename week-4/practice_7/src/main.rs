use std::io;

fn main() {
    println!("Please enter a number: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut number:i32 = input1.trim().parse().expect("failed to input");

    while number < 20{
        println!("Inside loop Number value is: {}", number);
        number+=1;
    }
    println!("Outside loop Number value is : {}", number);
}