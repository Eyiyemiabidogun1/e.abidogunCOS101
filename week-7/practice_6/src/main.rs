fn main() {
    let mut num = 5;
    mutate_num_to_zero(&mut num);
    println!("The value of num = {}", num);
}
fn mutate_num_to_zero(param_num:&mut i32){
    *param_num=*param_num*3;
    println!("param_num value is = {}", param_num);
}