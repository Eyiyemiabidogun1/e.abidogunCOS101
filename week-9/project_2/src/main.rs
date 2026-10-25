use std::io::Write;
use std::io::Read;  

fn main() {
    let student_name = vec!["Oluchi Mordi,", "Adams Aliyu,", "Shania Bolade,", "Adekunle Gold,", "Blanca Edemoh,"];
    let matric_number = vec!["ACC10211111,", "ECO10110101,", "CSC10328828,", "EEE11020202,", "MEE10202001,"];
    let department = vec!["Accounting,", "Economics,", "Computer,", "Electrical,", "Mechanical,"];
    let level = vec!["300,\n", "100,\n", "200,\n", "200,\n", "100,\n"];

    let mut file = std::fs::File::create("PAU-SMIS2.csv").expect("Failed to create");

    file.write_all("Student Name,Matric Number,Department,Level\n".as_bytes()).expect("Failed to write");

    for i in 0..student_name.len() {
        file.write_all(student_name[i].as_bytes()).expect("failed to write");
        
        file.write_all(matric_number[i].as_bytes()).expect("failed to write");
        
        file.write_all(department[i].as_bytes()).expect("failed to write");
        
        file.write_all(level[i].as_bytes()).expect("failed to write");
        
    }
    let mut file_to_read = std::fs::File::open("PAU-SMIS2.csv").expect("Failed to open for reading");

    let mut m = String::new();
    file_to_read.read_to_string(&mut m).expect("failed to read");
    
    println!("\nHere is the file content:");
    println!("{}", m);
}