use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    //Assigning values
    
    println!("Enter a ");
    io::stdin().read_line(&mut input1).expect("Inavlid");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter b");
     io::stdin().read_line(&mut input2).expect("Inavlid");
     let b: f32 = input2.trim().parse().expect("Not a valid number");

     println!("Enter c");
     io::stdin().read_line(&mut input3).expect("Inavlid");
     let c: f32 = input3.trim().parse().expect("Not a valid number");
     //Quadratic formula = -(b) +/- sqrt((b^2)-(4)(a)(c))/2(a)

     let d: f32 = b*2.0 - 4.0*a*c;
     println!("The discriminant is {}",d );

     if d > 0.0{
         let root1:f32 = (-(b) + d.sqrt()) / (2.0 * a);
         let root2:f32 = (-(b) - d.sqrt()) / (2.0 * a);
         println!("The roots are distict \nThey are {} and {}",root1, root2 );
     }

     else if d == 0.0{
        let root1:f32 = (-(b) + d.sqrt()) / (2.0 * a);
        let root2:f32 = (-(b) - d.sqrt()) / (2.0 * a);
        println!("There is only one real root");
     }
     else if d < 0.0{
        println!("There are no real roots");
     }







}
