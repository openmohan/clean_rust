use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");

    println!("Please enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read the line");
    println!("your input is {input}");

    let random = rand::thread_rng().gen_range(1..=100);
    println!("the random number is {random}");
}
