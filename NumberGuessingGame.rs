use rand::prelude::*;
use std::io;

fn main() {

    let rand_num = thread_rng().gen_range(1..101);
    println!("Guess a number between 1 and 100. You have 5 tries.");
    let mut lives: u32 = 5;
    let mut count: u32 = 0;

    loop {
        let mut user_guess = String::new();
        io::stdin().read_line(&mut user_guess).expect("Failed to read input");
        let user_guess: u32 = user_guess.trim().parse().unwrap();


            if user_guess > rand_num {
                println!("Your number is too high, guess again.");
                count += 1;
                lives -= 1;

                if count == 1 {
                    println!("You have used {} guess.", count);
                } else {
                    println!("You have used {} guesses.", count);
                }

            } else if user_guess < rand_num {
                println!("Your number is too low, guess again.");
                count += 1;
                lives -= 1;

                if count == 1 {
                    println!("You have used {} guess.", count);
                } else {
                    println!("You have used {} guesses.", count);
                }

            } else {
                println!("Correct! You win!");
                break;
            }
	    
        if lives == 1 {
            println!("You have {} life left!", lives);
        } else if lives == 0 {
            println!("You have {} lives remaining. You lose!", lives);
            break;
        } else {
            println!("You have {} lives remaining.", lives);
        }
	    
    }
}

