use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number :)");

    let secrect_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
