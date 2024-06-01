use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number :)");

    let secrect_num = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secrect_num}");

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u32 = guess.trim().parse().expect("Please enter a number");

        println!("Your guess was: {guess}");

        match guess.cmp(&secrect_num) {
            Ordering::Less => println!("To less!"),
            Ordering::Greater => println!("To much!"),
            Ordering::Equal => {
                println!("You Won!!!");
                break;
            }
        }
    }
}
