use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your height(in centimeters):");
    io::stdin().read_line(&mut input).expect("Invalid string");
    let height:f32 = input.trim().parse().expect("Invlid Number");

    if height >= 150.0 && height <= 170.0 {
        println!("You are of average height person");
    }
    else if height > 170.0 && height <= 195.0 {
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.00 {
        println!("You are dwarf");
    }
    else {
        println!("abnormal height");
    }
}