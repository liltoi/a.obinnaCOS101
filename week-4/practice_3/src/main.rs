fn main() {
    let name1 = "Obinna Amaechi";
    println!("My father's name is {}", name1);


    //find and replace
    let name2 = name1.replace("Obinna","Michael");
    println!("You can also call him {}",name2 );
    let faculty = "Faculty of Arts and Crafts";
    println!("I am a student of the {}",faculty );


    //find and replace
    let school = faculty.replace("Arts","Science");
    println!("I am also a student of the {}",school );



}
