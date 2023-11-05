fn main() {
   let name = "Obinna Amaechi George";
   let department = "Software Engineering";
   let uni = "Covanent Unievrsity";


   let mut school = "School of Science".to_string();
   //push string
   school.push_str(" and Crafts");

   println!("My name is {}",name );
   //check lenght
   println!("The lenght of my full name is {}",name.len()  );
   println!("I am a student of {} Department",department );
   println!("{}",school );
   println!("{}",uni );



}

