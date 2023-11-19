use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut yes = String::new();

    println!("Enter Employee First and Last name");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter employee age");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let a:i32 = input2.trim().parse().expect("Please enter a valid age");

    println!("Is the Employee experienced (yes or no):");
    io::stdin().read_line(&mut yes).expect("Failed to read line");
    let experience = yes.trim();
    


    let annual_incentive = match (experience, a){
        ("yes",a) if a >= 40 =>1560000,
        ("yes",a) if a >= 30 && a <=40 => 1480000,
        ("yes",a) if a < 28 => 1300000,
        ("no", _) => 100000, 
        _=>{
            println!("Invalid input");
            return;
        }
    };

    println!("The anual incentive of the employee is: N{}",annual_incentive );






}
