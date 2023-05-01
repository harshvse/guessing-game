use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the Number Game");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter a number please:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Big"),
        }
    }
}
