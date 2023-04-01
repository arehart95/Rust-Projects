use rand::prelude::*;
use std::io;

fn main() {

    let rand_num = thread_rng().gen_range(1..101);
    println!("Enter a number between 1 and 100.");

    loop {
        let mut user_guess = String::new();
        io::stdin().read_line(&mut user_guess).expect("Failed to read input");
        let user_guess: u32 = user_msg.trim().parse().unwrap();

        if user_num > rand_num {
            println!("Your number is too high, guess again.");
        } else if user_num < rand_num {
            println!("Your number is too low, guess again.");
        } else {
            println!("Correct! You win!");
            break;
        }
    }
}
