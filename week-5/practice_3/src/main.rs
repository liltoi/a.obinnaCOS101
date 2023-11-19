//Rust program to read the height of a peron
//and then print if person is tall,dwarf,
//or average height of person

use std::io;
 fn main(){

    let mut input = String::new();

    println!("Enter your height (in centimeters):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height :f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <=170.0
    {
        println!("You ai for your height");
    }
    else if height >=100.0 && height <=150.0
    {
        println!("You are short bro");
    }
    else if height >170.0 && height <=210.00
    {
        println!("You are actually a giant");
    }

 
}