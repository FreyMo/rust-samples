extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("This is guess the number");
    println!("Enter your guess!");

    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("Random: {}", secret_number);

    io::stdin()
        .read_line(&mut guess)
        .expect("Expected a result. Failed.");

    println!("You guessed {}", guess);
}
