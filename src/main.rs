use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number :)");

    let secrect_num = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secrect_num}");

    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");

    println!("Your guess was: {guess}");
}
