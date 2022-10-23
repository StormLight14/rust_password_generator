use std::io;
use rand::seq::IteratorRandom;

fn main() {
    //Create &str full of characters to randomly choose
    let printable = r##"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!"#$%&'()*+, -./:;<=>?@[\]^_`{|}~ "##;
    let mut password_length = String::new();

    println!("Enter the length of password you want generated: ");

    //Get user input for pass length; Change user input to a u32
    io::stdin().read_line(&mut password_length).expect("Failed to read line.");
    let password_length: u32 = password_length.trim().parse().expect("Please enter a number.");
    let mut password = String::new();

    //Go through the password length and
    for _num in 0..password_length {
        let mut rng = rand::thread_rng();
        let rand_char = printable.chars().choose(&mut rng);
        match rand_char {
            Some(char) => password.push(char),
            None       => ()
        }
    }
    println!("{}", password);
}

