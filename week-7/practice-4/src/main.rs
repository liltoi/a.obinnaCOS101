use std::io;

fn add(a: i32, b: i32) {

    let sum = a + b;

    println!("Sum of A and B = {}", sum );

}

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();


    println!("Enter input paramteres for A:");
    io::stdin().read_line(&mut input1).expect("Failed to input");
    let a:i32 = input1.trim().parse().expect("Invalid input");


    println!("Enter input paramters for B:");
    io::stdin().read_line(&mut input2).expect("Failed to input");
    let b: i32 = input2.trim().parse().expect("Invalid input");

    //call add function with arguments
    add(a,b);


}