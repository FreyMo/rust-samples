extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is guess the number");

    loop {
        println!();
        println!("Enter your guess!");

        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1, 20);

        io::stdin()
            .read_line(&mut guess)
            .expect("Expected a result. Failed.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(reason) => {
                println!("{}", reason);
                continue;
            }
        };
        println!("You guessed {}", guess);
        println!("The secret number was {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Correct! You won!");
                break;
            }
        }
    }
}
