fn main() {
    let full_name = "Pan-atlantic University";
    println!();
    println!("Name: {}", full_name);
    println!();
    println!("Before Trim ");
    println!("Length is {}", full_name.len());
    println!();
    println!("After trim:");
    println!("Length is: {}", full_name.trim().len());
}
