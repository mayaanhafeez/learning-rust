use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    loop {
        let secret_number = rand::rng().random_range(1..=100);

        println!("the secret number is {secret_number}");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error reading input");

        let guess: u32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => {
                println!("you win bro");
                break;
            }
        }
    }
}
