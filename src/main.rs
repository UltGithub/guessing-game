use std::cmp::Ordering;
use std::io;

use clap::Parser;
use guessing_game::{check_guess, parse_guess};
use rand::prelude::*;

/// A command-line number guessing game.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Upper bound of the secret number (the range is 1..=MAX)
    #[arg(short, long, default_value_t = 100, value_parser = clap::value_parser!(u32).range(2..))]
    max: u32,

    /// Maximum number of attempts (unlimited if not set)
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..))]
    attempts: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let secret_number = rand::rng().random_range(1..=args.max);
    let mut tries: u32 = 0;

    println!("Guess the number between 1 and {}!", args.max);
    loop {
        if let Some(limit) = args.attempts
            && tries >= limit
        {
            println!("Out of attempts! The number was {secret_number}.");
            break;
        }

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

        tries += 1;
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
