use rand::Rng;
use std::{cmp::Ordering, fmt::Display, io};

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

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess == "quit" {
                    break;
                }
                continue;
            }
        };
        let guess = Guess::new(guess);

        println!("You guessed: {guess}");

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            },
        }
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Your guess must be between 1 and 100");
        }

        Guess {
            value,
        }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

impl Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
