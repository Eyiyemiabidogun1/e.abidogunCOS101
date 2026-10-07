fn main() {
    let arr1:[i32;4] = [10,20,30,40];
    println!("\nArray with data type");
    println!("array is {:?}", arr1);
    println!("array size is {}",arr1.len());

    let arr2 = [10.4,9.2,8.6,3.5,2.9,99.1];
    println!("\nArray without datatype");
    println!("array is {:?}", arr2);
    println!("array size is {}", arr2.len());

    let arr3:[i32;8] = [-1;8];
    println!("\nArray without datatype");
    println!("array is {:?}", arr3);
    println!("array size is {}", arr3.len());
}
