fn main() {
    
    //Using Vec::new();
    let v : Vec<f64> = Vec::new();

    //println the size of the vector
    println!("\nThe lenght of Vec::new is {}",v.len());

    //Using macro
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    println!("\nThe lenght of vec macro is {}",v.len() );
}
