use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MIN: u32 = 1;
const MAX: u32 = 100;

pub fn guessing() {
    println!("Guess the number between {} and {}", MIN, MAX);
    println!("Please input your guess.");

    let secret = rand::thread_rng().gen_range(MIN, MAX + 1);
    let mut num_guess = 0;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        num_guess += 1;
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }

    println!("It took {} guesses", num_guess)
}
