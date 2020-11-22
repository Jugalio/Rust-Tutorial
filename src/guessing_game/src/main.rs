use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: u8,
}

impl Guess {
    pub fn new(value: u8) -> Guess {
        if value < 1 || value > 100 {
            panic!("{} is not allowed as a Guess. Choose a number between 1 and 100.")
        }

        Guess { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101) as u8;

    println!("Guess a number between 1 and 100");

    loop {
        let mut guess = String::new();
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Invalid guess");
                continue;
            }
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
