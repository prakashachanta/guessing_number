use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Welcome to Guessing Number game!");
    println!("I have picked a number betweeen 1 & 100, can you guess it?");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Guess the number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to readline");

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_)=> {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("Your guessed number is {:?}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater=> println!("Too Large"),
            Ordering::Equal => {
                println!("Got it right");
                break;
            }
        }
    }
}
