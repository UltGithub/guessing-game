use std::cmp::Ordering;
use std::io;

use guessing_game::{check_guess, parse_guess};
use rand::prelude::*;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    println!("Guess the number!");
    loop {
        println!("Please input your guess.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess = match parse_guess(&input) {
            Ok(num) => num,
            Err(_) => {
                println!("Please type whole number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match check_guess(guess, secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too bing!"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }
    }
}
