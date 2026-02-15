use rand::Rng;
use std::{cmp::Ordering, io};

/// A game where you have to guess a number between 0 and 100 (both included).
fn main() {
    let mut rng = rand::thread_rng();
    let secret_number: u32 = rng.gen_range(0..=100);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please input a number between 0 and 100 (both included).");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
