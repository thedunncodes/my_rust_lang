use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn read_line() {
    println!("Hello, welcome to the great guessing game!\nGuess a random number!\n");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Secret number: {secret_number}");

    loop {
        println!("Please make a guess!\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.\nToo bad, no guess made :(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!!\n");
                continue;
            }
        };

        println!("Your guess is: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}

fn main() {
    read_line();
}
