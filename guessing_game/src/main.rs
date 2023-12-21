use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Give me a number from 1 to 10");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("The secret number is: {secret_number}");

    let mut guess = String::new();
    loop {
        guess.clear();
        match io::stdin().read_line(&mut guess) {
            Ok(n) => {
                println!("{n} bytes read");
                println!("{guess}");
                if guess.ends_with("\r\n") {
                    println!("Windows-style newline found (\\r\\n)");
                }
                // Check for Unix-style newline
                else if guess.ends_with("\n") {
                    println!("Unix-style newline found (\\n)");
                }
                // No newline found
                else {
                    println!("No newline found");
                }
            }
            Err(error) => println!("error: {error}"),
        }

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You have entered {}", guess);
    }
}
