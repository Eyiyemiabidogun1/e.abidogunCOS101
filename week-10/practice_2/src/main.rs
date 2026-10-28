fn main() {
    let v = vec![10,9,8];
    let v2 = v;

    display(v2);

    
}

fn display(v:Vec<i32>){
    println!("{:?}", v);
}