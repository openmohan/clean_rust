use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    println!("Please enter a number:");

    let random = rand::thread_rng().gen_range(1..=100);
    // println!("the random number is {random}");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read the line");
        println!("your input is {input}");
        let input: i32 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("error");
                continue;
            }
        };

        match input.cmp(&random) {
            Ordering::Less => println!("lesser"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("success");
                break;
            }
        }
    }
}
