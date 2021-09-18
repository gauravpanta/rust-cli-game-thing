use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(0..101);
    println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            }
        }
    }
}
