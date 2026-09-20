use std::io;

fn main() {
    let mut input = String::new();
    let mut input2 = String::new();

    println!("CHOOSE YOUR EXPIRIENCE
    1 Expirienced
    2 Not Expirienced");
    
    io::stdin().read_line(&mut input).expect("Invalide input");
    let choice1: u8 = input.trim().parse().expect("Invalid input (try a number)");

    println!("enter your age:");
    io::stdin().read_line(&mut input2).expect("invalid input");
    let choice2: u8 = input2.trim().parse().expect("Invalid input (try a number)");

    let c1: bool; 
    if choice1 == 1 {
        c1 = true;
    } else {
        c1 = false;
    }

    if c1 == true && choice2 >= 40 {
        println!("Your annual incentive is 1,560,000!!!");
    } else if c1 == true && choice2 > 29 && choice2 < 40 { 
        println!("Your annual incentive is 1,480,000!!!");
    } else if c1 == true && choice2 < 28 {
        println!("Your annual incentive is 1,300,000");
    } else if c1 == false {
        println!("Your annual incentive is 100,000");
    }
}
