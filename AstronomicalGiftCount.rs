use std::io;
use num_traits::pow; // imported from crates.io

fn count_gift(age: u32) -> u32 {
    
    let gifts: u32;

    match age % 2 { 
        0 => gifts = pow(age, 3),
        1 => gifts = pow(age, 2),
        _ => unreachable!()
    };

    gifts
}

fn main() {
    println!("Enter your age: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Couldn't get input.");

    let age: u32 = buffer.trim().parse().unwrap();
    println!("You are {} years old, and therefore eligible for {} gifts!", age, count_gift(age));


}
