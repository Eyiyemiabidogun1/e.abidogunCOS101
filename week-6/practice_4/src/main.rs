fn main() {
    let full_name = "Tolu Eyiyemi Abidogun";
    let department = "Software Engineering";
    let uni = "Pan-atlantic University";


    let mut school = "School of Science".to_string();
    school.push_str("and Technology");

    println!("My name is: {}", full_name);
    println!("The length of my full_name is: {}", full_name.len());
    println!("I am a student of the {} department", department);
    println!("{}", school);
    println!("{}", uni);
}
