use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret is {secret_number}");

    loop {
        println!("Please guess the number.");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                if guess == "quit" {
                    break;
                }
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            },
        }
    }
}
