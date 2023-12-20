use std::io;

fn main() {
    println!("Give me a number from 1 to 10");

    let mut guess = String::new();

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

    println!("You have entered {}", guess.trim());
}
