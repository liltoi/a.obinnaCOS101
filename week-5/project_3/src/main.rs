            break;
    }



    let mut password = String::new();

    println!("Enter valid password");
    io::stdin().read_line(&mut password).expect("Failed to read ");
    let password = password.trim().to_lowercase();

    if password.len() > 3 || password.len() >= 8 || password.chars().all(|c| c.is_ascii_lowercase() && 
        c.is_alphanumeric() && 
        c.is_digit(10)) {

        println!("Valid Password");

    }

    if password.chars().any(|c|"$@#".contains(c)) {

        println!("Not a valid password");
    }

    else {

        println!("Not a valid password");
    }
