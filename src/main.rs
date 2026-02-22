use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("{}", "Guess the number!".bright_blue().bold());

    loop {
        print!(
            "{}",
            "Enter difficulty level (1: 1-10, 2: 1-25, 3: 1-50, 4: 1-100): ".yellow()
        );
        io::stdout().flush().expect("Failed to flush stdout");

        let mut difficulty = String::new();
        io::stdin()
            .read_line(&mut difficulty)
            .expect("Failed to read line");

        let max_number = match difficulty.trim() {
            "1" => 10,
            "2" => 25,
            "3" => 50,
            "4" => 100,
            _ => {
                println!("{}", "Invalid input, defaulting to level 1 (1-10).".red());
                10
            }
        };

        let mut rng = rand::rng();
        let secret_number = rng.random_range(1..=max_number);

        loop {
            print!("{}", "Please input your guess: ".yellow());
            io::stdout().flush().expect("Failed to flush stdout");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", "Please enter a valid number!".red());
                    continue;
                }
            };

            println!("You guessed: {}", guess.to_string().cyan());

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "That's too small!".magenta()),
                Ordering::Greater => println!("{}", "That's too big!".magenta()),
                Ordering::Equal => {
                    println!("{}", "You win!".bright_green().bold());
                    break;
                }
            }
        }

        print!("{}", "Play again? (y/n): ".yellow());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut again = String::new();
        io::stdin()
            .read_line(&mut again)
            .expect("Failed to read line");
        if again.trim().to_lowercase() != "y" {
            break;
        }
    }
}
