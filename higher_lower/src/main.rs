use std::io;
use rand::prelude::*;
use std::process::exit;
fn main() {
    let secret_num = rand::thread_rng().gen_range(1..101);

    println!("I have chosen a number between 1 andd 100..\nGuess the number!");
    let mut guess = 0;
    while guess != secret_num {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input line.");
        let guess: u32 = input.trim().parse().expect("Failed to parse input.");

        if guess > secret_num {
            println!("\n{} is too high! Guess lower.", guess);
        } else if guess < secret_num {
            println!("\n{} is too low! Guess Higher!", guess);
        } else {
            println!("\nYOU GUESSED RIGHT! The secret number was {}.", secret_num);
            break;
        }
    }
    exit(0);
}
