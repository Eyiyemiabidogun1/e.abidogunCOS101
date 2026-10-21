use std::io::Write;
fn main() {
    let mut file = std::fs::File::create("data.csv").expect("failed to create file");
    file.write_all("lager,stout,non-alcoholic\n".as_bytes()).expect("failed to write");
    file.write_all("33 export, legend, maltina\n".as_bytes()).expect("Failed to write");
    file.write_all("desperados,Turbo king, amstel malta\n".as_bytes()).expect("Failed to write");
    file.write_all("Goldberg,Willams,malta Gold\n".as_bytes()).expect("Failed to write");
    file.write_all("Gudler,,fayrouz\n".as_bytes()).expect("failed to write");
    file.write_all("Heineken,,\n".as_bytes()).expect("failed to write");
    file.write_all("Star,,\n".as_bytes()).expect("failed to write");
    print!("file has been written");
}
