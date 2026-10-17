use std::io;
fn main() {
    let mut years: Vec<(String,i32)> = Vec::new();
    println!("WELCOME TO THE Ernst & Young(EY) Screening interview");
    let mut input1 = String::new();
    println!("How many applicants are there?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let no_user:i32 = input1.trim().parse().expect("failed to register integer");
    for _x in 0..no_user{
    // this is to get the name of the applicant
    let mut input2 = String::new();
    println!("What is your Name?");
    io::stdin().read_line(&mut input2).expect("failed to register input");
    let y = input2.trim();
        // this is to get the years of expirience
        let mut input3 = String::new();
        println!("How many years have you worked in your field(eg. 14)");
        io::stdin().read_line(&mut input3).expect("failed to register input");
        let exp:i32 = input3.trim().parse().expect("invalid input");
        // this is a vector of tuples that holds 2 values the string is for the name and the i32 is for the exp
        years.push((y.to_string(),exp));
    }
    let mut max_years = years[0].1;
    let mut max_name = &years[0].0;

    for year in &years{
        if year.1 > max_years{
        max_years = year.1;
        max_name = &year.0;
    }
    }
    println!("The applicant with the most amount of expirience is {} with {} years of expirience",max_name,max_years);
}
