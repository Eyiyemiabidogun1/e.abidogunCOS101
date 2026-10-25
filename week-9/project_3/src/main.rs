use std::io::Read;
use std::io::Write;
fn main() {
    let commissioners = vec![
    "commissioners",
    "\nAigbogun Alamba Daudu,", 
    "\nMurtala Afeez Bendu,", 
    "\nOkorocha Calistus Ogbona,", 
    "\nAdewale Jimoh Akanbi,", 
    "\nOsazuwa Faith Etieye\n,"
];

let ministries = vec![
    "\nministries",
    "\nInternal Affairs,", 
    "\nJustice,", 
    "\nDefense,", 
    "\nPower & Steel,", 
    "\nPetroleum\n,"
];

let zones = vec![
    "\nzones",
    "\nSouth West,", 
    "\nNorth East,", 
    "\nSouth South,", 
    "\nSouth West,", 
    "\nSouth East,"
];

let mut com_file = std::fs::File::create("commissioners.csv").expect("Failed to create");
let mut min_file = std::fs::File::create("ministries.csv").expect("failed to create");
let mut zones_file= std::fs::File::create("zones.csv").expect("failed to create");

for x in 0..commissioners.len(){
    com_file.write_all(commissioners[x].as_bytes()).expect("failed to write_all");
}
for y in 0..ministries.len(){
    min_file.write_all(ministries[y].as_bytes()).expect("failed to write_all"); 
}
for z in 0..zones.len(){
    zones_file.write_all(zones[z].as_bytes()).expect("failed to write_all");
}
let mut com_string = String::new();
let mut min_string = String::new();
let mut zon_string = String::new();

let mut com_open = std::fs::File::open("commissioners.csv").expect("failed to open");
let mut min_open = std::fs::File::open("ministries.csv").expect("failed to open");
let mut zon_open = std::fs::File::open("zones.csv").expect("failed to open");

com_open.read_to_string(&mut com_string).expect("failed to read");
min_open.read_to_string(&mut min_string).expect("failed to read");
zon_open.read_to_string(&mut zon_string).expect("failed to read");
let mut master_file = std::fs::File::create("master.csv").expect("failed to create");

master_file.write_all(com_string.as_bytes()).expect("failed to write");
master_file.write_all(",".as_bytes()).expect("failed to write_all");
master_file.write_all(min_string.as_bytes()).expect("failed to write");
master_file.write_all(",".as_bytes()).expect("failed to write_all");
master_file.write_all(zon_string.as_bytes()).expect("failed to write");




}