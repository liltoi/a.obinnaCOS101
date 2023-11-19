// Rust program to determie age pass

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter yout name:" );
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    println!("Enter your age ");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let age : i32 = input2.trim().parse().expect("Not a vlaid number");

    if age >= 18 {
        println!("Welcome to the party {}",input1 );
    }else {
        println!("You don't sabi yet {}Go to your home. "
            ,input1);

    }

}