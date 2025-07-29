use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("----Welcome to the Guessing Game----");
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue; //it will restart the loop if invalid number
            }
        };

        println!("You guessed : {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! try again."),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number!");
                break;
            }
        }
    }
}
