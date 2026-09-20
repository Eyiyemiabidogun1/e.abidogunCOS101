use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter A :");
    io::stdin().read_line(&mut input1).expect("failed to register input");
    let a:f32 = input1.trim().parse().expect("failed to register input (try a number)");

    println!("Enter B :");
    io::stdin().read_line(&mut input2).expect("failed to register input");
    let b:f32 = input2.trim().parse().expect("failed to register input (try a number)");

    println!("Enter C :");
    io::stdin().read_line(&mut input3).expect("failed to register input");
    let c:f32 = input3.trim().parse().expect("failed to register input (try a number)");

    let q:f32 = b*b - 4.0*a*c;
    let pq:f32 = (-b  + q.sqrt()) / 2.0*a;
    let nq:f32 = (-b - q.sqrt())/ 2.0*a;

    if q > 0.0  {
        println!("root1 is = {} , root2 is = {}", pq,nq);
        }else if q == 0.0 {
             let root: f32 = -b / (2.0 * a);
        println!("One real root = {}", root);
        }
        else {
            println!("There are no real values of this equation");
        }
    }