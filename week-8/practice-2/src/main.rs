fn main() {
    
    let v = vec!["c","o","m","p","u","t","e","r"];

    let mut input1 = String::new();

    println!("Enter index value from 0-7");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index: usize  = input1.trim().parse().expect("Invalid input");

    let chr  = v[index];


    println!("{} is the character for index[{}]\n",chr, index );
}
