use std::io;

fn character(){
    let mut input1 = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lttr:char = input1.trim().parse().expect("Invalid input");

    if lttr >= '0' && lttr <= '9'
    {
        println!("It is a digit");           
    }
    else{

        println!("It is not a digit");
    }
}



fn main(){


    //calling function
    println!("Welcome This program checks wheter a character variable contains a digit or not");
    character()
}
