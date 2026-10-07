use std::io;
fn trap_area(h:i32,b1:i32,b2:i32){
    let trap_area:i32 = h /  2* (b1 + b2);
    println!("Your Trapezium area is = {}", trap_area);
}
fn rhom_area(d1:f32,d2:f32){
    let rhom_area:f32 = 0.5 * d1 * d2;
    println!("Your Rhombus area is = {}", rhom_area);
}
fn parall_area(ba:i32,al:i32){
    let parall_area:i32 = ba * al;
    println!("Your Parallelogram area is = {}", parall_area);
}
fn cube_suf_area(side:i32){
    let cube_suf_area:i32 = 6 * side * side;
    println!("Your cubes surface area is = {}", cube_suf_area);
}
fn cylinder_volume(radi:f64, height:f64){
    let cylinder_volume:f64 = 3.142*radi*radi*height;
    println!("Your Cylinders volume = {}", cylinder_volume);
}

fn main() {
   let mut _op = true;
   while _op == true {
    println!("========HELLO!!!! WELCOME TO THE SHAPE CALCULATOR========="); 
    println!("What would you Like to calculate
        \n1. Trapezium area(enter 1)
        \n2. Rhombus area(enter 2)
        \n3. Parallelogram area(enter 3)
        \n4. Cube surface area (enter 4)
        \n5. Cylinder volume(enter 5)
        \n6. leave the CALCULATOR") ;
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to register input");
    let i:i32 = input1.trim().parse().expect("Please input an integer");

    if i == 1 {
        println!("Enter value for height: ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("failed to register input");
        let h:i32 = input2.trim().parse().expect("failed to register use integer");

         println!("Enter value for base 1: ");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("failed to register input");
        let b1:i32 = input3.trim().parse().expect("failed to register use integer");

         println!("Enter value for base 2: ");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("failed to register input");
        let b2:i32 = input4.trim().parse().expect("failed to register use integer");

        trap_area(h,b1,b2)

    }else if i == 2 {
        println!("Enter value for diagonal 1: ");
        let mut input5 = String::new();
        io::stdin().read_line(&mut input5).expect("failed to register input");
        let d1:f32 = input5.trim().parse().expect("failed to register use integer");

          println!("Enter value for diagonal 2: ");
        let mut input6 = String::new();
        io::stdin().read_line(&mut input6).expect("failed to register input");
        let d2:f32 = input6.trim().parse().expect("failed to register use integer");

        rhom_area(d1,d2)
    }else if i == 3{
          println!("Enter value for base area: ");
        let mut input7 = String::new();
        io::stdin().read_line(&mut input7).expect("failed to register input");
        let ba:i32 = input7.trim().parse().expect("failed to register use integer");

          println!("Enter value for altitude: ");
        let mut input8 = String::new();
        io::stdin().read_line(&mut input8).expect("failed to register input");
        let al:i32 = input8.trim().parse().expect("failed to register use integer");
        parall_area(ba,al)
    }else if i == 4 {
          println!("Enter value for side: ");
        let mut input9 = String::new();
        io::stdin().read_line(&mut input9).expect("failed to register input");
        let side:i32 = input9.trim().parse().expect("failed to register use integer");
        cube_suf_area(side)
    }else if i == 5 {
          println!("Enter value for radius: ");
        let mut input10 = String::new();
        io::stdin().read_line(&mut input10).expect("failed to register input");
        let radi:f64 = input10.trim().parse().expect("failed to register use integer");

          println!("Enter value for height: ");
        let mut input11 = String::new();
        io::stdin().read_line(&mut input11).expect("failed to register input");
        let height:f64 = input11.trim().parse().expect("failed to register use integer");
        cylinder_volume(radi,height)
    }else if i == 6{
        _op = false;
    }else {
        println!("Please input a correct value either 1,2,3,4,5 or 6");
    }
}
}