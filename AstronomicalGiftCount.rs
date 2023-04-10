use std::io;

fn count_gift(age: u32) -> u32 {
    let mut gifts: u32 = 0;

    if age > 0 {
        if age % 2 == 0 {
            gifts = age * age * age;
        } else {
            gifts = age * age;
        }
    }
    gifts
}

fn main() {

    println!("Enter your age: ");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Couldn't get input.");

    let age: u32 = buffer.trim().parse().unwrap();
    println!("You entered: {}. You are eligible for {} gifts!", age, count_gift(age));


}
