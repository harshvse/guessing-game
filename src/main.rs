use std::io;

fn main() {
    println!("Guess the Number Game");
    println!("Enter a number please:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Cannot read the line");

    println!("You guessed {guess}");
}
