extern crate rand;

use std::io; // import std io
use rand::Rng;
use std::cmp::Ordering;

fn greetings() {
    println!("Guess the number!");
    println!("Please input your guess (1-101).");
}

fn main() {
    let secret = rand::thread_rng().gen_range(1,101);

    loop {
        greetings();
        let mut guess = String::new(); // immutable guess variable
        io::stdin().read_line(&mut guess) // read line
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }


        }
    }


}
